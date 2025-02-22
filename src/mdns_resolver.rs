use libmdns::Responder;

pub async fn start() {
    let responder = Responder::new().unwrap();
    let _ = responder.register("_aod._tcp.local".into(), "AOD (Android On Desktop)".into(), 8000, &["path=/"]);
}