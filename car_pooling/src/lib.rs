pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut points = [0; 1000];

    for trip in trips.iter() {
        let (passengers, from, to) = (trip[0], trip[1], trip[2]);
        for i in from..to {
            points[i as usize] += passengers;
            if points[i as usize] > capacity {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let trips = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let result = car_pooling(trips, 4);
        assert_eq!(result, false);
    }

    #[test]
    fn it_works2() {
        let trips = vec![vec![2, 1, 5], vec![3, 5, 7]];
        let result = car_pooling(trips, 3);
        assert_eq!(result, true);
    }
}
