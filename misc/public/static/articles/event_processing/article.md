# Event processing

To process an event is to change its state from unprocessed to processed. An event may consist of other events, so to say that an event processing has succeeded requires at least the successful processing of all its constituent events.

An event processing may expectedly (1) succeed or (2) fail; or (3) the information about the processing state may not reach the recipient due to imperfect conditions, such as networking; or (4) an event may only partially succeed due to a bug, in other words—unexpectedly terminate. Since (1) and (2) are expected scenarios that have achieved their objectives and were successfully observed, I will look into cases (3) and (4) some more.

If the information about the event processing did not reach you, you would want to try the operation again.

If the event processing partially succeeded or unexpectedly terminated, you would want to try the operation again, perhaps after fixing a bug.

The problem to solve is one of repeatability, or idempotency. Most often, to make an operation repeatable, it would only require making the constituent operations repeatable.

Since the operation may only partially succeed, such a partial state is to be tolerable at least in the short term; in the long term, it may be ignored or later found and rolled back or canceled.

## But how do you make an irreducible operation repeatable/idempotent?

According to Eric Normand, there are data, calculations, and side effects.

When we talk about operations, they may be either calculations or side effects.

Calculations do not leave traces; you may repeat them, and nothing durable changes. For example: an expression 2 + 2.

Side effects leave traces; they include variable mutations, disk writes, and network calls.

A few general operations:

- **Update an object (`object.update(...)`)**: Repeatable  
  - Example: hashmap set key-value pair.
  - Example: update user info by user ID.

- **Create an object (`object.post()`)**: Often, to make it repeatable, you would need to handle the case when the object already existed.  
  - Example: hashmap set key-value pair.
  - Example: create a bucket.
  
- **Delete an object (`object.delete()`)**: Often, to make it repeatable, you would need to handle the case when the object did not exist.  
  - Example: delete a bucket.

There are too many cases to exhaust, so only ask a question for your own problems: how do I make this piece repeatable?

Furthermore, you may extend this model not only to a state machine with two states (unprocessed/processed) but to a state machine with many more states.
