# pong
An experiment to implement Atari's classic 1972 arcade game Pong with networked multiplayer support in Rust using Piston.

## Run
1. install [Rust](https://www.rust-lang.org/tools/install)
2. clone this repository
```
> git clone https://github.com/sathwikmatsa/pong.git
```
3. navigate to the source repository and cargo run
```
> cd pong/
> cargo run
```

## Instructions
- *Step 1*: Press 'S' or 'C' to act as a server or client respectively. None of which matters, unless you have a publicly routable IP address.

![welcome screen](https://user-images.githubusercontent.com/30603669/70903547-1f589800-2025-11ea-9b56-db4539b7d2e1.png)

***
- *Step 2*:
  - if you press "S" (server), pass on the IP/PORT displayed on the screen to the other player
  
  ![server screen](https://user-images.githubusercontent.com/30603669/70904371-2b455980-2027-11ea-9718-c7587cbf409f.png)
  - if you press "C" (client), enter IP/PORT of the server and press **ENTER**
  
  ![client screen](https://user-images.githubusercontent.com/30603669/70904478-7b242080-2027-11ea-873a-67367acda17b.png)
  
  Note: If the server doesn't have a [publicly routable IP address](https://networkengineering.stackexchange.com/a/40336), ensure computers of both the players connected to the same LAN.
***
- *Step 3*: Use **UP**/**DOWN** arrow keys to move the paddle. *Server* gets the *right* paddle. *Client* gets the *left* paddle.

![game play](https://user-images.githubusercontent.com/30603669/70904779-42d11200-2028-11ea-8241-af639d0d52e2.png)

## Testing
Tested locally on Windows.

## Todo
- [ ] Add sound effects
- [ ] Gameplay: max points, endless
- [ ] Handle program flow when either of players quit
- [ ] Develop Pong AI for use in single player
- [ ] Provide native installers

***
Please file issues or bug reports at https://github.com/sathwikmatsa/pong/issues
