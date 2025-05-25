use proto::frontend::{Signal as FrontendSignal, SignalAction};
use sysinfo::{Pid, Signal};

use crate::client::BackendContext;

pub fn process_signal(mut ctx: BackendContext, action: SignalAction) {
    let sys = &mut ctx.system();

    let signal = match action.signal {
        FrontendSignal::Kill => Signal::Kill,
        FrontendSignal::Pause => Signal::Stop,
        FrontendSignal::Term => Signal::Term,
        FrontendSignal::Resume => Signal::Continue,
    };

    let Some(proc) = sys.system.process(Pid::from_u32(action.pid)) else {
        return;
    };

    proc.kill_with(signal);
}
