use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;

const HALL_SIZE: usize = 11;
const NUM_ROOMS: usize = 4;
const ROOM_HALLWAY_ENTRANCE: [usize; 4] = [2, 4, 6, 8];
static COSTS: [usize; 4] = [1, 10, 100, 1000];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State<const N: usize> {
    cost: usize,
    position: [u8; N],
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

fn parse_input<const N: usize>(input: &str) -> [u8; N] {
    let mut start = [0; N];
    let room_size = (N - HALL_SIZE) / NUM_ROOMS;
    for (idx, b) in input
        .bytes()
        .filter(|&b| (b'A'..=b'D').contains(&b))
        .enumerate()
    {
        start[HALL_SIZE + (room_size * (idx % NUM_ROOMS)) + (idx / NUM_ROOMS)] = b - b'A' + 1;
    }
    start
}

fn try_gen_path_cost(
    position: &[u8],
    mut hall: usize,
    mut target_slot: usize,
    room_size: usize,
    amph: u8,
) -> Option<usize> {
    if hall > target_slot {
        std::mem::swap(&mut hall, &mut target_slot);
    }
    let room_number = (target_slot - HALL_SIZE) / room_size;
    let room_slot = (target_slot - HALL_SIZE) % room_size;
    let entrance = ROOM_HALLWAY_ENTRANCE[room_number];
    let start;
    let end;
    if entrance < hall {
        start = entrance;
        end = hall
    } else {
        start = hall + 1;
        end = entrance
    }
    if (1..=room_slot).all(|r| position[target_slot - r] == 0)
        && position[start..end].iter().all(|&p| p == 0)
    {
        let cost =
            (room_slot + 1 + (entrance.max(hall) - entrance.min(hall))) * COSTS[amph as usize - 1];
        return Some(cost);
    }
    None
}

fn gen_moves<const N: usize>(heap: &mut BinaryHeap<State<N>>, position: [u8; N], prev_cost: usize) {
    let room_size = (N - HALL_SIZE) / NUM_ROOMS;
    let get_room_start = |amph: u8| HALL_SIZE + ((amph as usize - 1) * room_size);
    let room_is_empty_or_same_kind = |room: usize| {
        let room_number = (room - HALL_SIZE) / room_size;
        let room_start = room_number * room_size + HALL_SIZE;
        position[room_start..room_start + room_size]
            .iter()
            .all(|&r| r == 0 || r as usize == room_number + 1)
    };
    let can_move_into_room = |amph: u8| {
        let room_start = get_room_start(amph);
        room_is_empty_or_same_kind(room_start)
    };
    let get_first_empty_slot = |amph: u8| {
        let room_start = get_room_start(amph);
        position[room_start..room_start + room_size]
            .iter()
            .zip(room_start..room_start + room_size)
            .rev()
            .find(|&(&r, _)| r == 0)
    };
    for hall_pos in [0, 1, 3, 5, 7, 9, 10] {
        let pos_state = position[hall_pos];
        if pos_state > 0 {
            if !can_move_into_room(pos_state) {
                continue;
            }
            if let Some((_, slot)) = get_first_empty_slot(pos_state) {
                let cost = try_gen_path_cost(&position, hall_pos, slot, room_size, pos_state);
                if cost.is_none() {
                    continue;
                }
                let mut new_position = position;
                new_position.swap(hall_pos, slot);
                heap.push(State {
                    cost: prev_cost + cost.unwrap(),
                    position: new_position,
                });
            }
        } else {
            for slot in
                (HALL_SIZE..N).filter(|&r| position[r] > 0 && !room_is_empty_or_same_kind(r))
            {
                let cost = try_gen_path_cost(&position, hall_pos, slot, room_size, position[slot]);
                if cost.is_none() {
                    continue;
                }
                let mut new_position = position;
                new_position.swap(hall_pos, slot);
                heap.push(State {
                    cost: prev_cost + cost.unwrap(),
                    position: new_position,
                });
            }
        }
    }
}

fn shortest_path<const N: usize>(start: State<N>) -> Option<usize> {
    let mut lowest_costs = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(start);
    while let Some(State { cost, position }) = heap.pop() {
        if position.windows(2).all(|i| i[0] <= i[1]) {
            return Some(cost);
        }
        if lowest_costs.get(&position).unwrap_or(&usize::MAX) < &cost {
            continue;
        }
        lowest_costs.insert(position, cost);
        gen_moves(&mut heap, position, cost);
    }
    None
}

fn main() -> std::io::Result<()> {
    let contents = include_str!("input.txt");
    let part1_position = parse_input::<{ 11 + 4 * 2 }>(contents);
    let start_state = State::<{ 11 + 4 * 2 }> {
        cost: 0,
        position: part1_position,
    };
    let mut part2_input = contents.lines().collect_vec();
    part2_input.splice(3..3, ["  #D#C#B#A#", "  #D#B#A#C#"]);
    let part2_position = parse_input::<{ 11 + 4 * 4 }>(&part2_input.join(""));

    let part1_cost = shortest_path::<{ 11 + 4 * 2 }>(start_state).unwrap();
    println!("{}", part1_cost);
    let part2_cost = shortest_path::<{ 11 + 4 * 4 }>(State::<{ 11 + 4 * 4 }> {
        cost: 0,
        position: part2_position,
    })
    .unwrap();
    println!("{}", part2_cost);
    Ok(())
}
