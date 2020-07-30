
#[cfg(test)]
mod tests {
    use crate::activity_pub_ext::MockAPProfile;
    use crate::activity_pub_ext::{APProfile, ApActor, SocialGraph, HolochainExpression, Expression, InterDNA, MethodPair};

    #[test]
    fn test_profile() {
        let ctx = MockAPProfile::create_profile_context();
        ctx.expect().returning(|actor_data| Ok(actor_data));
        println!("{:?}", MockAPProfile::create_profile(ApActor {
            context: String::from("https://www.w3.org/ns/activitystreams"),
            prefered_username: String::from("test"),
            inner: activitystreams::actor::Person::new(),
            inbox_pub: None, 
            outbox_pub: None, 
            followers_pub: None, 
            following_pub: None, 
            likes_pub: None,
            streams_pub: activitystreams::collection::Collection {
                items: None,
                total_items: Some(0),
                current: None,
                first: None,
                last: None,
                //This should be our own Object type where AnyBase would have the possiblity to be a holochain resource record 
                inner: activitystreams::object::Object {
                    attachment: Option<OneOrMany<AnyBase>>,
                    attributed_to: Option<OneOrMany<AnyBase>>,
                    audience: Option<OneOrMany<AnyBase>>,
                    content: Option<OneOrMany<AnyString>>,
                    summary: Option<OneOrMany<AnyString>>,
                    url: Option<OneOrMany<AnyBase>>,
                    generator: Option<OneOrMany<AnyBase>>,
                    icon: Option<OneOrMany<AnyBase>>,
                    image: Option<OneOrMany<AnyBase>>,
                    location: Option<OneOrMany<AnyBase>>,
                    tag: Option<OneOrMany<AnyBase>>,
                    start_time: Option<XsdDateTime>,
                    end_time: Option<XsdDateTime>,
                    duration: Option<XsdDuration>,
                    published: Option<XsdDateTime>,
                    updated: Option<XsdDateTime>,
                    in_reply_to: Option<OneOrMany<AnyBase>>,
                    replies: Option<OneOrMany<AnyBase>>,
                    to: Option<OneOrMany<AnyBase>>,
                    bto: Option<OneOrMany<AnyBase>>,
                    cc: Option<OneOrMany<AnyBase>>,
                    bcc: Option<OneOrMany<AnyBase>>,
                    inner: Base<Kind>,
                }
            },
            inbox_private: None, 
            outbox_private: None, 
            followers_private: None, 
            following_private: None, 
            likes_private: None,
            streams_private: activitystreams::collection::Collection {
                items: None,
                total_items: Some(0),
                current: None,
                first: None,
                last: None,
                inner: activitystreams::object::Object {
                }
            }
        }))
    }
}
