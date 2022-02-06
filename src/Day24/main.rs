/*
Stack operations:
    PUSH input[0] + 0
    PUSH input[1] + 6
    PUSH input[2] + 4
    PUSH input[3] + 2
    PUSH input[4] + 9
    input[5] == POP(input[4] + 9) - 2
    PUSH input[6] + 10
    input[7] == POP(input[6] + 10) - 15
    input[8] == POP(input[3] + 2) - 10
    PUSH input[9] + 6
    input[10] == POP(input[9] + 6) - 10
    input[11] == POP(input[2] + 4) - 4
    input[12] == POP(input[1] + 6) - 1
    input[13] == POP(input[0] + 0) - 1

Simplified:
    input[5] = input[4] + 7
    input[7] = input[6] - 5
    input[8] = input[3] - 8
    input[10] = input[9] - 4
    input[11] = input[2]
    input[12] = input[1] + 5
    input[13] = input[0] - 1

Largest: 94992994195998
Smallest: 21191861151161
*/
fn main() -> std::io::Result<()> {
    Ok(())
}
