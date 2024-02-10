use super::*;

mod limit_output {
	use super::*;

	#[test]
	fn works_with_default_input_without_limit() {
		let res = limit_output_old(Vec::new(), None);

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_default_input_with_limit() {
		let res = limit_output_old(Vec::new(), Some(10));

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn returns_full_input_when_limit_larger() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(10));

		assert_eq!(res, input);
	}

	#[test]
	fn returns_full_input_when_limit_none() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), None);

		assert_eq!(res, input);
	}


	#[test]
	fn works_with_limit_0() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(0));

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_limit_1() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(1));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_2() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(2));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_3() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(3));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_4() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(4));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_5() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_old(input.clone(), Some(5));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}
}

mod limit_output_only_positive {
	use super::*;

	#[test]
	fn works_with_default_input_without_limit() {
		let res = limit_output_only_positive_old(Vec::new(), None);

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_default_input_with_limit() {
		let res = limit_output_only_positive_old(Vec::new(), Some(10));

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn returns_full_input_when_limit_larger() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_positive_old(input.clone(), Some(10));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn returns_full_input_when_limit_none() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_positive_old(input.clone(), None);

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
		]);
	}


	#[test]
	fn works_with_limit_0() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_positive_old(input.clone(), Some(0));

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_limit_1() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_positive_old(input.clone(), Some(1));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_2() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_positive_old(input.clone(), Some(2));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_3() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_positive_old(input.clone(), Some(3));

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
		]);
	}
}

mod limit_output_only_negative {
	use super::*;

	#[test]
	fn works_with_default_input_without_limit() {
		let res = limit_output_only_negative_old(Vec::new(), None);

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_default_input_with_limit() {
		let res = limit_output_only_negative_old(Vec::new(), Some(10));

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn returns_full_input_when_limit_larger() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_negative_old(input.clone(), Some(10));

		assert_eq!(res, vec![
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn returns_full_input_when_limit_none() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_negative_old(input.clone(), None);

		assert_eq!(res, vec![
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
		]);
	}


	#[test]
	fn works_with_limit_0() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_negative_old(input.clone(), Some(0));

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_limit_1() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_negative_old(input.clone(), Some(1));

		assert_eq!(res, vec![
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_2() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_negative_old(input.clone(), Some(2));

		assert_eq!(res, vec![
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_limit_3() {
		let input: Vec<(u32, Dataset)> = vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		];

		let res = limit_output_only_negative_old(input.clone(), Some(3));

		assert_eq!(res, vec![
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
		]);
	}
}

mod sort_output {
	use super::*;

	#[test]
	fn works_with_default_input() {
		let res = sort_output_old(OldIntermediateChartData::default());

		assert_eq!(res, Vec::new());
	}

	#[test]
	fn works_with_single_dataset() {
		let input = OldIntermediateChartData {
			datasets: vec![
				(0, Dataset {
					label: String::new(), data: vec![
						DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},
						DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},
						DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},
					]
				})
			].into_iter().collect()
		};
		let res = sort_output_old(input);

		assert_eq!(res, vec![(0, Dataset {
			label: String::new(), data: vec![
				DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},
				DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},
				DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},
			]
		})]);
	}

	#[test]
	fn works_with_many_datasets() {
		let input = OldIntermediateChartData {
			datasets: vec![
				(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
				(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
				(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
				(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
				(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
				(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
				(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
				(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
				(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			].into_iter().collect()
		};
		let res = sort_output_old(input);

		assert_eq!(res, vec![
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}

	#[test]
	fn works_with_duplicates() {
		let input = OldIntermediateChartData {
			datasets: vec![
				(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
				(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
				(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
				(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
				(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
				(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
				(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
				(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
				(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			].into_iter().collect()
		};
		let res = sort_output_old(input);

		assert_eq!(res, vec![
			(2, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 200.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(5, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()}]}),
			(1, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 400.0, label: String::new()}]}),
			(7, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 300.0, label: String::new()}]}),
			(8, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 900.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 250.0, label: String::new()}]}),
			(6, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.1, label: String::new()}]}),
			(0, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()},DataPoint {name: None, timestamp: None, value: 100.0, label: String::new()}]}),
			(3, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -50.0, label: String::new()}]}),
			(4, Dataset {label: String::new(), data: vec![DataPoint {name: None, timestamp: None, value: 500.0, label: String::new()},DataPoint {name: None, timestamp: None, value: -90.0, label: String::new()}]}),
		]);
	}
}

mod get_date_for_period {
	use super::*;

	#[test]
	fn daily_period() {
		let date_period = "daily";
		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

		assert_eq!(res, expected_res);
	}

	#[test]
	fn monthly_period() {
		let date_period = "monthly";

		for i in 1..12 {
			let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, i, 5).unwrap();
			
			let res = get_date_for_period(date_period, timestamp);
			let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, i, 1).unwrap();
	
			assert_eq!(res, expected_res);
		}
	}

	#[test]
	fn quarterly_period() {
		let date_period = "quarterly";

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 2, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 3, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 5, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 6, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 8, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 11, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 12, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 1).unwrap();
		assert_eq!(res, expected_res);
	}

	#[test]
	fn yearly_period() {
		let date_period = "yearly";
		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 5).unwrap();

		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();

		assert_eq!(res, expected_res);
	}
}