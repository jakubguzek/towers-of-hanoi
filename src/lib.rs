/// ASCII art representing generic Towers of Hanoi
pub const TOWERS: &str = "
      |          |          |
      _          |          |
     ___         |          |
    _____        |          |
  _________  ____|____  ____|____
      1          2          3    ";

/// Recursively calculates steps needed to solve Towers of Hanoi with n disks.
/// It does so by accumulating the steps to `acc` vector. Steps are represented
/// as a tuple whose first item represents start rod and second represents end rod.
///
/// This function can consume a lot of memeory as it uses a vecor of u8 tuples to
/// remember the steps.
///
/// # Arguments
///
/// * `n`     - A number of disks,
/// * `start` - A rod from which we want to move the disks,
/// * `end`   - A rod to which we want to move the disks,
/// * `acc`   - A accumulator vector to which steps are pushed.
///
/// # Ecample
///
/// ```
/// // Returns a number of steps needed to solve a Towers of Hanoi with `n` disks.
/// use hanoi::hanoi_with_accumulation;
/// fn hanoi_steps(n: u32) -> u128 {
///     2u128.pow(n) - 1
/// }
/// // Allocate memory for vector with capacity of at least a number of expected steps.
/// let mut steps: Vec<(u8, u8)> = Vec::with_capacity(hanoi_steps(20).try_into().unwrap());
/// hanoi_with_accumulation(20, 1, 3, &mut steps);
/// for step in steps {
///     println!("{} -> {}", step.0, step.1);
/// }
/// ```
pub fn hanoi_with_accumulation(n: u32, start: u8, end: u8, acc: &mut Vec<(u8, u8)>) {
    // Base case.
    if n == 1 {
        acc.push((start, end))
    } else {
        let other = 6 - (start + end);
        hanoi_with_accumulation(n - 1, start, other, acc);
        acc.push((start, end));
        hanoi_with_accumulation(n - 1, other, end, acc)
    }
}

/// Recursively calculates steps needed to solve Towers of Hanoi with n disks
/// and prints them to stdout.
///
/// # Arguments
///
/// * `n`     - A number of disks,
/// * `start` - A rod from which we want to move the disks,
/// * `end`   - A rod to which we want to move the disks,
pub fn hanoi(n: u32, start: u8, end: u8) {
    // Base case.
    if n == 1 {
        print_step((start, end));
    } else {
        let other = 6 - (start + end);
        hanoi(n - 1, start, other);
        print_step((start, end));
        hanoi(n - 1, other, end)
    }
}

/// Returns a number of steps needed to solve a Towers of Hanoi with `n` disks.
pub fn hanoi_steps(n: u32) -> u128 {
    2u128.pow(n) - 1
}

/// Prints a formatted Towers of Hanoi step.
fn print_step(step: (u8, u8)) -> String{
    let step = format!("{} -> {}", step.0, step.1);
    println!("{}", step);
    step
}

// Unit tests.
#[cfg(test)]
mod test {
    use super::{hanoi_steps, print_step, hanoi_with_accumulation};

    #[test]
    fn test_hanoi_steps() {
        assert_eq!(hanoi_steps(4), 15);
    }

    #[test]
    fn test_print_step() {
        assert_eq!(print_step((1, 3)), "1 -> 3");
        assert_eq!(print_step((10, 32)), "10 -> 32");
    }
    
    #[test]
    fn test_hanoi_with_accumulation() {
        let correct_steps = vec![(1,3), (1,2), (3,2), (1,3), (2,1), (2,3), (1,3)];
        let mut calculated_steps: Vec<(u8, u8)> = Vec::with_capacity(correct_steps.len());
        hanoi_with_accumulation(3, 1, 3, &mut calculated_steps);
        assert_eq!(correct_steps, correct_steps)
    }
}
