use subscription_app::SubscriptionApp;

mod subscription;
mod subscription_app;
mod tui;
mod tui_entry;
mod visit;
fn main() {
    let app = SubscriptionApp::new();
}

#[cfg(test)]
mod tests {

    use std::io;

    use self::{subscription::Subscription, tui_entry::Entry, visit::Visit};

    use super::*;

    #[test]
    fn it_works() {
        let mut app = SubscriptionApp::new().unwrap();
        app.buy_subscription(subscription::SubscriptionType::Exercise12);
        app.visited(visit::VisitType::CrossFit);
        app.visited(visit::VisitType::Functional);
        app.save_content().unwrap();
    }
}
