use containers::entities::EntityService;
use containers::tapes::TapeService;
use containers::listeners::ListenerService;

#[derive(Debug)]
pub struct Game<S> {
    pub state: S,
    pub listeners: ListenerService,
    pub entities: EntityService,
    pub storage: TapeService,
}
