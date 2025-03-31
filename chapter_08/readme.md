cd examples
cargo new task-execution

cd src
cargo check
cargo run

/*

Available tasks: ["Task1", "Task2", "Task3", "Task4"]
Task2: Starting...
Task3: Starting...
Task1: Starting...
Task4: Starting with 6 rounds...
Task2: Starting...
Task4: Round 1 of 6
Task4: Round 2 of 6
Task4: Round 3 of 6
Task4: Round 4 of 6
Task4: Round 5 of 6
Task4: Round 6 of 6
Task4: Finished.
Task Task4: Duration: 606.555792ms
Task1: Finished.
Task Task1: Duration: 1.005085375s
Task2: Finished.
Task Task2: Duration: 4.001474375s
Task2: Finished.
Task Task2: Duration: 4.003569334s
Task3: Finished.
Task Task3: Duration: 5.004623125s
All tasks completed.
*/