package main

import (
	"math"
	"os"
	"text/template"
)

type data struct {
	Abscissa []float64
	Weights  []float64
	H        float64
}

func main() {
	h := math.Pow(2.0, -6.0) // arbitrary, but pretty safe
	p := 6                   // digits of precision
	eta := math.Pow10(-p)
	n := solveN(h, eta, p)
	wts := make([]float64, 2*n+1)
	absc := make([]float64, 2*n+1)
	for i, j := 0, -n; j <= n; j++ {
		wts[i] = weight(h, float64(j))
		absc[i] = abscissa(h, float64(j))
		i++
	}
	d := data{
		Abscissa: absc,
		Weights:  wts,
		H:        h,
	}
	f, _ := os.Create("tanhsinh_consts.rs")
	tmpl := `
// GENERATED CODE DO NOT EDIT

const WEIGHTS: [f64; {{len .Weights}}] = [{{range .Weights}}{{.}},{{end}}];
const ABSCISSA: [f64; {{len .Abscissa}}] = [{{range .Abscissa}}{{.}},{{end}}];
const H: f64 = {{.H}};

// END GENERATED CODE`

	t := template.Must(template.New("tmpl").Parse(tmpl))
	t.Execute(f, d)

}

func abscissa(h, j float64) float64 {
	return math.Tanh(math.Pi / 2 * math.Sinh(h*j))
}

func weight(h, j float64) float64 {
	return math.Pi / 2 * math.Cosh(h*j) / math.Pow(math.Cosh(math.Pi/2*math.Sinh(h*j)), 2.0)
}

func solveN(h, eta float64, p int) int {
	var solutions []int
	n := 1
	for n < 1000000 {
		if abscissa(h, float64(n))*math.Pow10(p) < 1.0*math.Pow10(p) && weight(h, float64(n)) <= eta {
			solutions = append(solutions, n)
		}
		n++
	}
	return solutions[len(solutions)-1]
}
