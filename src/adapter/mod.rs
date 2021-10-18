mod round_hole;
mod round_peg;
mod square_peg;
mod square_peg_adapter;

#[cfg(test)]
mod tests {
    use crate::adapter::round_hole::RoundHole;
    use crate::adapter::round_peg::RoundPeg;
    use crate::adapter::square_peg::SquarePeg;
    use crate::adapter::square_peg_adapter::SquarePegAdapter;

    #[test]
    fn test0(){
        let hole = RoundHole::new(5.0);
        let rpeg = RoundPeg::new(5.0);
        hole.fits(rpeg);

        let small_sqpeg = SquarePeg::new(5.0);
        let large_sqpeg = SquarePeg::new(10.0);
        // hole.fits(small_sqpeg); // compile error

        let small_sqpeg_adapter = SquarePegAdapter::new(Box::new(small_sqpeg));
        let large_sqpeg_adapter = SquarePegAdapter::new(Box::new(large_sqpeg));
        hole.fits(small_sqpeg_adapter);
        hole.fits(large_sqpeg_adapter);
    }
}