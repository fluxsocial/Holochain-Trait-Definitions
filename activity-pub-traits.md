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
    resource: Option<String> //eg get_by_address
    method: Method
    params: Option<JsonString>
}

pub struct ApActor<T: activitystreams::Actor, CT: activitystreams::Collection> {
    context: String, //Likely need to define our own context that extends from Activity Streams to incorporate pub/private resources
    actor: T,
    //Likely that all DNA methods below would point to social contexts
    //References to public DNA's
    //Since auth is not possible on DNA's and instead they are protected by membrane rules; we need different DNA's for different privacy levels
    inbox_pub: Option<DnaMethod>, //Post @ social context
    outbox_pub: Option<DnaMethod>, //Post @ social context 
    followers_pub: Option<DnaMethod>, //Read links @ social context with by_agent = id
    following_pub: Option<DnaMethod>, //Read links @ social context with by_agent = id
    collections_public: CT, //Various collections of expressions/social contexts
    //References to private DNA's
    inbox_private: Option<DnaMethod>, //Post @ social context
    outbox_private: Option<DnaMethod>, //Post @ social context
    followers_private: Option<DnaMethod>, //Read links @ social context with by_agent = id
    following_private: Option<DnaMethod>, //Read links @ social context with by_agent = id
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
    fn my_followers(relationship: Option<String>) -> activitystreams::Collection;
    fn followers(followed_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::Collection;
    fn nth_level_followers(n: uint, followed_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::Collection;

    fn my_followings(relationship: Option<String>) -> activitystreams::Collection;
    fn following(following_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::Collection;
    fn nth_level_following(n: uint, following_agent: activitystreams::actor::Actor, relationship: Option<String>) -> activitystreams::Collection;

    fn follow(other_agent: activitystreams::actor::Actor, relationship: Option<String>) -> Result<(), ZomeApiError>;
    fn unfollow(other_agent: activitystreams::actor::Actor, relationship: Option<String>) -> Result<(), ZomeApiError>;

    // Connection Related Operations (i.e. bidirectional friendship)
    fn my_friends() -> activitystreams::Collection;
    fn friends_of(agent: activitystreams::actor::Actor) -> activitystreams::Collection;

    fn request_friendship(other_agent: activitystreams::actor::Actor);
    fn decline_friendship(other_agent: activitystreams::actor::Actor);

    fn incoming_friendship_requests() -> activitystreams::Collection;
    fn outgoing_friendship_requests() -> activitystreams::Collection;

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
