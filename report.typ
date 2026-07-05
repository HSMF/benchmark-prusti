#let data = json("stats.json")
#let data_any_good = json("stats-only-any-success.json")

#let get_by_suite(vals, suite) = {
  vals.filter(x => x.suite == suite).at(0)
}
#let suite_name(suite) = {
  suite.replace(regex("(-overflows)?(-z3|cvc5)?$"), "")
}
#let suite_prover(suite) = {
  if suite.contains(regex("z3|cvc5")) {
    suite.replace(regex("(?:.*)-(z3|cvc5)$"), x => x.captures.at(0))
  } else {
    "z3"
  }
}
#let suite_overflows(suite) = {
  if suite.contains("overflows") {
    sym.checkmark
  } else {
    sym.crossmark
  }
}

#let foo(means, failures) = {
  let skip = failures.filter(x => x.total == 1).map(x => x.suite)
  let best(arr, hm) = {
    arr.filter(x => not skip.contains(x.suite)).map(x => x.at(hm)).fold(arr.at(0).at(hm), (x, y) => calc.min(x, y))
  }
  let failure_rates = failures.map(x => {
    x.rate = x.failures / x.total
    x
  })
  let hl(best, content) = if content == best { [*#content*] } else { [#content] }
  let best_mean = best(means, "mean")
  let best_median = best(means, "median")
  let best_stddev = best(means, "stddev")
  let best_rate = best(failure_rates, "rate")
  table(
    columns: 8,
    stroke: none,
    align: (auto, center, center),
    table.header(
      [Implementation], [Prover], [CO], [Mean [ms]], [Median [ms]], [Stddev [ms]], [Failures], [Failure Rate]
    ),
    table.hline(),
    ..means
      .filter(x => not skip.contains(x.suite))
      .map(x => {
        let fail = get_by_suite(failure_rates, x.suite)
        (
          suite_name(x.suite),
          suite_prover(x.suite),
          suite_overflows(x.suite),
          hl(best_mean, x.mean),
          hl(best_median, x.median),
          hl(best_stddev, x.stddev),
          [#fail.failures / #fail.total],
          [
            #show: it => if fail.rate == best_rate { [*#it*] } else { it }
            #calc.round(fail.rate, digits: 2)%],
        )
      })
      .flatten(),
  )
}
#let filter(data, f) = {
  (data.means.filter(f), data.failures.filter(f))
}

CO = checked overflows

= Arithmetic-only Benchmarks

== Arithmetic

#foo(..filter(json("stats/arith.json"), x => true))


== Arithmetic, dataset without overflows

#foo(..filter(json("stats/arith-no-overflow.json"), x => x.suite.contains("overflows")))

== Arithmetic, dataset without overflows, all passing

#foo(..filter(json("stats/arith-no-overflow-strict.json"), x => x.suite.contains("overflows")))

= Bitwise-only Benchmarks

== Bitwise

#foo(..filter(json("stats/bitwise.json"), x => true))


== Bitwise, dataset without overflows

#foo(..filter(json("stats/bitwise-no-overflow.json"), x => x.suite.contains("overflows")))

== Bitwise, dataset without overflows, all passing

#foo(..filter(json("stats/bitwise-no-overflow-strict.json"), x => x.suite.contains("overflows")))

= Mixed Benchmarks

== Mixed

#foo(..filter(json("stats/mixed.json"), x => true))


== Mixed, dataset without overflows

#foo(..filter(json("stats/mixed-no-overflow.json"), x => x.suite.contains("overflows")))

== Mixed, dataset without overflows, all passing

#foo(..filter(json("stats/mixed-no-overflow-strict.json"), x => x.suite.contains("overflows")))

