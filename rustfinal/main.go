package main

import (
	"log"
	"net/http"
	"sync"
	"time"
)

type Server struct {
	counter *sync.Mutex // Mutex to protect the shared counter
	value   int         // Shared state
}

func NewServer() *Server {
	return &Server{
		counter: &sync.Mutex{},
		value:   0,
	}
}

func (s *Server) handleRequest() {
	// Simulate handling a request with a sleep of 100ms
	time.Sleep(100 * time.Millisecond)

	// Lock the mutex and modify the counter
	s.counter.Lock()
	defer s.counter.Unlock()

	s.value++
	log.Printf("Handled request, counter: %d", s.value)
}

func (s *Server) start(numRequests int) {
	var wg sync.WaitGroup
	for i := 0; i < numRequests; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			s.handleRequest()
		}()
	}
	wg.Wait()
}

func (s *Server) makeRequest(url string) error {
	// Make an HTTP GET request
	resp, err := http.Get(url)
	if err != nil {
		log.Printf("Error making request to %s: %v", url, err)
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		log.Printf("Request to %s completed with status: %d", url, resp.StatusCode)
	} else {
		log.Printf("Request to %s failed with status: %d", url, resp.StatusCode)
	}
	return nil
}

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile) // Enable line number in logs

	log.Println("Program started.")

	server := NewServer()
	log.Println("Server initialized.")

	// Handling 500 concurrent requests
	server.start(500)

	// Simulating making an HTTP request
	err := server.makeRequest("https://www.rust-lang.org")
	if err != nil {
		log.Fatalf("Request failed: %v", err)
	}

	log.Println("Program finished.")
}
