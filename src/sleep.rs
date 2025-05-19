use chrono::{DateTime, Utc};
use errno::errno;
use libc;
use std::{mem::MaybeUninit, ptr};
use tokio::signal::unix::{signal, SignalKind};

#[derive(Debug)]
pub enum Error {
    Errno(errno::Errno),
    Io(std::io::Error),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<errno::Errno> for Error {
    fn from(err: errno::Errno) -> Self {
        Error::Errno(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Errno(errno) => write!(f, "{errno}"),
            Error::Io(error) => write!(f, "{error}"),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

unsafe fn arm_timer(duration: i64) -> Result<libc::timer_t> {
    // First, initialize our timer
    let mut timer: libc::timer_t = MaybeUninit::zeroed().assume_init();
    // this means we are going to create a SIGALRM
    let mut sev: libc::sigevent = MaybeUninit::zeroed().assume_init();
    sev.sigev_notify = libc::SIGEV_SIGNAL;
    sev.sigev_signo = SignalKind::alarm().as_raw_value();
    if libc::timer_create(libc::CLOCK_REALTIME, &mut sev, &mut timer) != 0 {
        return Err(Error::from(errno()));
    }

    // Now, get the time to sleep until
    let mut its = libc::itimerspec {
        it_interval: libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        it_value: MaybeUninit::zeroed().assume_init(),
    };
    // by getting the current time
    if libc::clock_gettime(libc::CLOCK_REALTIME, &mut its.it_value) != 0 {
        let err = Err(Error::from(errno()));
        disarm_timer(timer)?;
        return err;
    }

    // and changing the duration
    its.it_value.tv_sec += duration as libc::time_t;

    // Finally, arm the timer
    if libc::timer_settime(timer, libc::TIMER_ABSTIME, &its, ptr::null_mut()) != 0 {
        let err = Err(Error::from(errno()));
        disarm_timer(timer)?;
        return err;
    }

    Ok(timer)
}
unsafe fn disarm_timer(timer: libc::timer_t) -> Result<()> {
    if libc::timer_delete(timer) != 0 {
        return Err(Error::from(errno()));
    }
    Ok(())
}

#[cfg(unix)]
pub async fn sleep_until<Tz: chrono::TimeZone>(time: DateTime<Tz>) -> Result<()> {
    let time = time.with_timezone(&Utc);
    // we must schedule our signal handler before the first signal appears
    let mut alarm = signal(SignalKind::alarm())?;
    loop {
        let currtime = Utc::now();
        let seconds_to_sleep = (time - currtime).num_seconds();
        if seconds_to_sleep < 0 {
            break;
        }
        // now we set a timer for the specified date
        let timer = unsafe { arm_timer(seconds_to_sleep)? };
        // and wait for the signal
        alarm.recv().await;
        unsafe { disarm_timer(timer)? }
    }
    Ok(())
}
