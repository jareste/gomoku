42 project done in colaboration with [bielaltes](https://github.com/bielaltes) 

It's our first project working directly with an IA. For this project we implemented a minmax algorithm with alpha-beta pruning.

For the GUI we used [Beby library](https://bevyengine.org/).

While we were doing the project, we discarded lots of scopes such as: Running minmax on GPU, monte carlo tree search, diferent kind of heuristics evaluation or generating datasets to make ia work a bit easier. We discarded most of them just becouse of it's complexity or maybe we were not applying them properly, who knows.

Main rules:

    - Captures: you will be able to capture any pair of stones by closing them with yours in any direction. This will cause on a removal of the stones. Example: XOOX (capture) XOOOX (no capture).
    - Double Free-three: You are not allowed to do a movement that could potentially generate two free-threes. A free-three is a group of three stones that are alone. Example: -XXX- or -X-XX-.
    - Winning: You may achieve five in a row stones or capture 10 stones from your opponent.

Usage:
    - Cargo run.
        (We strongly recommend you to run it with "CARGO_TARGET_DIR=/tmp/gomoku/ cargo run", this will just generate the bin on /tmp folder, it's 3GB more or less.)

Menu:
    - 1vs1: you can play with your friends or against yourself.
    - 1vsIA: by default IA will start always first, making it harder to the player to win. (IA score is something like 40-1, good luck.)
    - IAvsIA: just a simulation of IA against our own IA. (second player IA was lazyly done, so you can expect lazy behaviour.)
    - Settings: There's an option to make IA start second.

While playing, you can also take profit of Hints. (As stated, second player IA was lazyly implemented so up to you to trust on it.).

Good luck and we hope you to enjoy our work!