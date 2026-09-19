package main

import (
	"fmt"
	"net/http"
	"sync"
	"time"
)

type Result struct {
	URL        string
	StatusCode int
	Duration   time.Duration
	Err        error
}

func check(url string, client *http.Client, results chan<- Result, wg *sync.WaitGroup) {
	defer wg.Done()

	start := time.Now()
	response, err := client.Get(url)
	duration := time.Since(start)

	if err != nil {
		results <- Result{URL: url, Duration: duration, Err: err}
		return
	}
	defer response.Body.Close()

	results <- Result{
		URL:        url,
		StatusCode: response.StatusCode,
		Duration:   duration,
	}
}

func main() {
	urls := []string{
		"https://example.com",
		"https://www.github.com",
		"https://httpbin.org/status/200",
	}

	client := &http.Client{Timeout: 5 * time.Second}
	results := make(chan Result, len(urls))

	var wg sync.WaitGroup
	for _, url := range urls {
		wg.Add(1)
		go check(url, client, results, &wg)
	}

	wg.Wait()
	close(results)

	for result := range results {
		if result.Err != nil {
			fmt.Printf("%-32s ERROR %v (%s)\n", result.URL, result.Err, result.Duration)
			continue
		}

		fmt.Printf("%-32s %d (%s)\n", result.URL, result.StatusCode, result.Duration)
	}
}
