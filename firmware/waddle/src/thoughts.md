# State

I want to easily find how long a key has been pressed to be able to do OnHold.
Right now when we scan we do `let s1 = scan(); delay(); let s2 = scan(); let curr_state = State::intersect(s1,s2);`
and that works fine. We can easily figure out the actual state.
But! What if instead of creating new states all the time we have *one* state where we have a set of currently pressed
keys, and for each `scan()` we do we `tick()` all the pressed keys.  
Like this
T0
tick()
State{}
T1
Button (0,0) is pressed
(0,0) is added to State with 0 ticks
tick()
State{((0,0),1)}
T2
Nothing happens
tick()
State{((0,0),2)}
T3
Button (2,4) is pressed
tick()
State{((0,0),3), ((2,4),1)}
T4
Button (0,0) is released
tick()
State{((2,4),2)}

etc. Then we can check the number of ticks and base our sending of keycodes based on that.  
Probably do `send()` right after `tick()`. And send will then have to build the actual keycodes that we should send.
Some keys can be normal keys, other OnHolds, so it's important to get the actual key that is to be used for the number
ticks the pos has.

How this plays with combos I'm not fully sure yet. Perhaps a combo is visible when a onhold activates. Hold A, after
holding A for 50ms, press B and the key C is sent.

I think this is the way forward. Could potentially reduce needed space as we are not creating and destroying states
all the time.
    
    