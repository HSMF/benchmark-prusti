python plot_benchmarks.py \
	benchmarks/bitwise-everywhere-mixed-overflows \
	benchmarks/tight-conversion-mixed-overflows \
	--prover cvc5 z3 --output-stats stats/mixed-no-overflow.json --filter "d.outputs == 0"

python plot_benchmarks.py \
	benchmarks/bitwise-everywhere-mixed \
	benchmarks/bitwise-everywhere-mixed-overflows \
	benchmarks/tight-conversion-mixed \
	benchmarks/tight-conversion-mixed-overflows \
	--prover cvc5 z3 --output-stats stats/mixed.json

python plot_benchmarks.py \
	benchmarks/bitwise-everywhere-bitwise-overflows \
	benchmarks/tight-conversion-bitwise-overflows \
	--prover cvc5 z3 --output-stats stats/bitwise-no-overflow.json --filter "d.outputs == 0"

python plot_benchmarks.py \
	benchmarks/bitwise-everywhere-bitwise \
	benchmarks/bitwise-everywhere-bitwise-overflows \
	benchmarks/tight-conversion-bitwise \
	benchmarks/tight-conversion-bitwise-overflows \
	--prover cvc5 z3 --output-stats stats/bitwise.json

python plot_benchmarks.py \
	benchmarks/baseline-arith-overflows \
	benchmarks/bitwise-everywhere-arith-overflows \
	benchmarks/tight-conversion-arith-overflows \
	--prover cvc5 z3 --output-stats stats/arith-no-overflow.json --filter "d.outputs == 0"

python plot_benchmarks.py \
	benchmarks/baseline-arith \
	benchmarks/baseline-arith-overflows \
	benchmarks/bitwise-everywhere-arith \
	benchmarks/bitwise-everywhere-arith-overflows \
	benchmarks/tight-conversion-arith \
	benchmarks/tight-conversion-arith-overflows \
	--prover cvc5 z3 --output-stats stats/arith.json
