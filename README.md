The best way to run this program is with the command:

```cargo run --package Programming-Assignment-2 --bin Programming-Assignment-2 --release <num_threads>```

Where `num_threads` is the number of threads to execute.

Problem 1 Strategy:

Pre-Game Agreement: The guests agree on a leader and a signaling mechanism (like eating the cupcake).

Leader's Role: The leader is responsible for replacing cupcakes when eaten. This action signifies that one more unique guest has visited the labyrinth.

Followers' Role: When a non-leader guest visits the labyrinth for the first time, they eat the cupcake if they find one. They never eat a cupcake again in subsequent visits.

Counting Mechanism: Leader keeps track of number of cupcakes they've replaced, when they've replaced N-1 cupcakes all guests have visited.


Problem 2 Strategy: 

I chose to implement the second strategy because it was similar to Problem 1 except all guests have the same role.

I designed a Guest structure with functions for entering and leaving the room. The threads all loop and with each iteration they try to enter.

If the Showroom is BUSY, they are denied entry, but if it is AVAILABLE they can enter, set the status to BUSY, and a timer puts the thread to sleep for
50ms. After the thread wakes up it calls the leave function, setting the status back to AVAILABLE.

As threads leave the showroom, their id is added to a hash set, and if the hashset length is equal to the number of guests the thread breaks from the loop.

