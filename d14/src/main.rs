    #![warn(
        bad_style,
        unused,
        unused_import_braces,
        unused_qualifications,
        unused_results
    )]

    #[derive(Debug)]
    struct Restroom {
        height: usize,
        width: usize,
        positions: Vec<(usize, usize)>,
        velocities: Vec<(i32, i32)>,
    }

    impl Restroom {
        fn new(input: &str) -> Self {
            let mut positions: Vec<(usize, usize)> = Vec::new();
            let mut velocities: Vec<(i32, i32)> = Vec::new();
            for line in input.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();

                let position: Vec<usize> = parts[0].split("=").collect::<Vec<&str>>()[1]
                    .split(",")
                    .map(|coordinate| coordinate.parse().unwrap())
                    .collect();
                positions.push((position[0], position[1]));

                let velocity: Vec<i32> = parts[1].split("=").collect::<Vec<&str>>()[1]
                    .split(",")
                    .map(|coordinate| coordinate.parse().unwrap())
                    .collect();
                velocities.push((velocity[0], velocity[1]));
            }

            let height = positions.iter().max_by_key(|(_, col)| col).unwrap().1 + 1;
            let width = positions.iter().max_by_key(|(row, _)| row).unwrap().0 + 1;

            Self {
                height,
                width,
                positions,
                velocities,
            }
        }

        fn patrol(&mut self) {
            for ((x, y), (dx, dy)) in self.positions.iter_mut().zip(self.velocities.iter()) {
                let current_x = *x as i32;
                let current_y = *y as i32;
                let width = self.width as i32;
                let height = self.height as i32;

                let mut new_x = current_x + dx;
                let mut new_y = current_y + dy;

                new_x = new_x.rem_euclid(width);
                new_y = new_y.rem_euclid(height);

                assert!(new_x >= 0 && new_x < width);
                assert!(new_y >= 0 && new_y < height);

                *x = new_x as usize;
                *y = new_y as usize;
            }
        }

        fn count_robots_in_quadrants(&self) -> [usize; 4] {
            let mut quadrants = [0; 4];
            let mid_x = self.width / 2;
            let mid_y = self.height / 2;

            for &(x, y) in &self.positions {
                if x == mid_x || y == mid_y {
                    continue;
                }

                let quadrant = match (x < mid_x, y < mid_y) {
                    (true, true) => 0,   // top-left
                    (false, true) => 1,  // top-right
                    (true, false) => 2,  // bottom-left
                    (false, false) => 3, // bottom-right
                };
                quadrants[quadrant] += 1;
            }

            quadrants
        }
        
        fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
            ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as usize
        }

        fn robots_distance(&self) -> usize {
            let mut dist = 0;
            
            for i in 0..self.positions.len() - 1 {
                for j in (i + 1)..self.positions.len() - 1 {
                    dist += Restroom::manhattan_distance(self.positions[i], self.positions[j]);
                }
            }
            dist
        }


        fn christmas_tree(&mut self, n: usize) -> usize {
            let mut min_dist = self.height * self.width * self.positions.len();
            let mut min_idx = 0;

            for i in 0..n {
                self.patrol();
                let dist = self.robots_distance();
                
                if dist < min_dist {
                    min_dist = dist;
                    min_idx = i;
                }
            }

            min_idx + 101   // + cycle
        }

    }



    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let input = aoe::read_input_file("input")?;
        let mut restroom = Restroom::new(&input);

        for _ in 0..100 {
            restroom.patrol();
        }

        let quadrants = restroom.count_robots_in_quadrants();
        let result_fst_part: usize = quadrants.iter().product();

        let result_snd_part: usize = restroom.christmas_tree(8200);

        println!("First part: {}, Second part: {}", result_fst_part, result_snd_part);

        Ok(())
    }
