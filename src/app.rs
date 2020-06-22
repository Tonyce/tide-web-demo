use crate::{middlewares, routers};

pub fn init_with_state<State: Send + Sync + 'static>(
    state: State,
) -> tide::Server<State> {
    let mut app = tide::Server::with_state(state);
    app.at("/health").get(|_| async { Ok("") });

    middlewares::app_middleware(&mut app);
    routers::app_routers(&mut app);

    app
}

pub fn init() -> tide::Server<()> {
    init_with_state(())
}
