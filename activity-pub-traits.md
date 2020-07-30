# Activity Pub x Junto Holochain Traits

## Profile Trait

```rust

pub enum Method {
    Post,
    Get,
    Put,
    Delete
}

/// Describes a DNA method 
pub struct DnaMethod {
    dna: Address,
    resource: Option<String>, //eg get_by_address
    method: Method,
    params: Option<hdk::prelude::JsonString> // Params for function
}

pub struct MethodPair {
    post: Option<DnaMethod>,
    get: Option<DnaMethod>
}

pub struct ApActor<T: activitystreams::Actor, CT: activitystreams::Collection> {
    context: String, //Likely need to define our own context that extends from Activity Streams to incorporate pub/private resources
    actor: T,
    //Likely that all DNA methods below would point to social contexts
    //References to public DNA's
    //Since auth is not possible on DNA's and instead they are protected by membrane rules; we need different DNA's for different privacy levels
    inbox_pub: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    outbox_pub: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    followers_pub: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    following_pub: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    collections_public: CT, //Various collections of expressions/social contexts
    //References to private DNA's
    inbox_private: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    outbox_private: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    followers_private: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    following_private: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    collections_private: CT //Various collections of expressions/social contexts
}

pub trait APProfile {
    fn create_profile(actor_data: ApActor) -> ZomeApiResult<ApActor>;
    fn get_profile(target: Address) -> ZomeApiResult<Option<ApActor>>;
    fn update_profile(actor_data: ApActor) -> ZomeApiResult<ApActor>;
    fn delete_profile() -> ZomeApiResult<()>;
}
```

## Social Graph

```rust
use dpki::DpkiRootHash;
type Identity = DpkiRootHash;

trait SocialGraph {
    // Follow Related Operations
    // Inner values for collections here likely Object of type relationship
    fn my_followers(relationship: Option<String>) -> activitystreams::OrderedCollection;
    fn followers(followed_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::OrderedCollection;
    fn nth_level_followers(n: uint, followed_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::OrderedCollection;

    fn my_followings(relationship: Option<String>) -> activitystreams::OrderedCollection;
    fn following(following_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::OrderedCollection;
    fn nth_level_following(n: uint, following_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::OrderedCollection;

    fn follow(other_agent: activitystreams::actor::Actor, relationship: Option<String>) -> Result<(), ZomeApiError>;
    fn unfollow(other_agent: activitystreams::actor::Actor, relationship: Option<String>) -> Result<(), ZomeApiError>;

    // Connection Related Operations (i.e. bidirectional friendship)
    fn my_friends() -> activitystreams::OrderedCollection;
    fn friends_of(agent: activitystreams::actor::Actor) -> activitystreams::OrderedCollection;

    fn request_friendship(other_agent: activitystreams::actor::Actor);
    fn decline_friendship(other_agent: activitystreams::actor::Actor);

    fn incoming_friendship_requests() -> activitystreams::OrderedCollection;
    fn outgoing_friendship_requests() -> activitystreams::OrderedCollection;

    fn drop_friendship(other_agent: activitystreams::actor::Actor) -> Result<(), ZomeApiError>;
}

```

## Expression Trait

```rust
/// A holochain expression
struct Expression<T: activitystreams::Object> {
    entry: Entry,
    headers: Vec<ChainHeader>,
    expression_dna: Address,
    activity_streams_entry: T,
    // @Nico - newly added in the case that (potential additions) below are not added. This provides a way for user
    // to specify a given Inter-DNA-Link-DNA they would like people to use for comments.
    // Nico: I wonder if we really need this - even without the addtions below.
    // Since we have `SocialContext` as defined above I don't think
    // we need below "potential addittions" add all.
    // This might be nice-to-have.
    inter_dna_link_dna: Option<Address>,
}

/// An interface into a DNA which contains Expression information. Expected to be interacted with using expression Addresses
/// retrieved from a social context or by using a Identity retreived from a users social graph.
/// In this situation you can see that the Expression DNA/trait does not need to include any index capability
/// as this is already infered to the agent by the place they got the expression from; social context or social graph.
///
/// If the expression should be private to a group of people then the host DNA should be membraned.
trait Expression {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String, inter_dna_link_dna: Option<Address>) -> Expression;
    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(author: Identity, count: uint, page: uint) -> Vec<Expression>;
    fn get_expression_by_address(address: Address) -> Option<Expression>;

    /// Send an expression to someone privately p2p
    fn send_private(to: Identity, content: String, inter_dna_link_dna: Option<Address>);
    /// Get private expressions sent to you
    fn inbox() -> Vec<Expression>;
}
```


## Inter-DNA

```rust
/// Entry marking a reference to some other entry in another DNA
struct GlobalEntryRef {
    dna_address: Address,
    entry_address: Address,
}

/// Interface for cross DNA links. Allows for the discovery of new DNA's/entries from a known source DNA/entry.
/// Host DNA should most likely implement strong anti spam logic if this is to be a public - unmembraned DNA.
trait InterDNA {
    fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> activitystreams::Relationship;
    fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> activitystreams::Relationship;

    fn get_outgoing(source: GlobalEntryRef, filter_dna: Address) -> activitystreams::Collection; //Relationship
    fn get_incoming(target: GlobalEntryRef, filter_dna: Address) -> activitystreams::Collection; //Relationship
}

```
