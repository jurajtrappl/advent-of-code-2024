from collections import deque
from dataclasses import dataclass
from typing import List, Set, Tuple
import heapq

@dataclass(frozen=True)
class State:
    x: int
    y: int
    direction: Tuple[int, int]  # (dx, dy)

def parse_maze(maze_str: str) -> List[List[str]]:
    return [list(line) for line in maze_str.strip().split('\n')]

def find_start_end(maze: List[List[str]]) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    start = end = None
    for y in range(len(maze)):
        for x in range(len(maze[0])):
            if maze[y][x] == 'S':
                start = (x, y)
            elif maze[y][x] == 'E':
                end = (x, y)
    return start, end

def get_next_directions(current_dir: Tuple[int, int]) -> List[Tuple[int, int]]:
    # All possible directions: right, down, left, up
    directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    current_idx = directions.index(current_dir)
    
    # Return current direction and adjacent directions (clockwise and counterclockwise)
    return [
        directions[current_idx],  # Continue straight
        directions[(current_idx + 1) % 4],  # Turn right
        directions[(current_idx - 1) % 4],  # Turn left
    ]

def solve_maze(maze_str: str) -> int:
    maze = parse_maze(maze_str)
    start, end = find_start_end(maze)
    
    # Priority queue elements: (score, State(x, y, direction))
    # Start facing east (1, 0)
    initial_state = State(start[0], start[1], (1, 0))
    pq = [(0, initial_state)]
    seen = set()
    
    while pq:
        score, state = heapq.heappop(pq)
        
        # Check if we've reached the end
        if (state.x, state.y) == end:
            return score
            
        # Skip if we've seen this state before
        if state in seen:
            continue
        seen.add(state)
        
        # Try all possible next moves
        for next_dir in get_next_directions(state.direction):
            new_x = state.x + next_dir[0]
            new_y = state.y + next_dir[1]
            
            # Check if move is valid
            if (0 <= new_y < len(maze) and 
                0 <= new_x < len(maze[0]) and 
                maze[new_y][new_x] != '#'):
                
                # Calculate new score
                new_score = score + 1  # Movement cost
                if next_dir != state.direction:
                    new_score += 1000  # Rotation cost
                
                new_state = State(new_x, new_y, next_dir)
                heapq.heappush(pq, (new_score, new_state))
    
    return float('inf')  # No path found

# Test with example mazes
example1 = """
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
""".strip()

example2 = """
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
""".strip()

print(f"Example 1 solution: {solve_maze(example1)}")  # Should output 7036
print(f"Example 2 solution: {solve_maze(example2)}")  # Should output 11048