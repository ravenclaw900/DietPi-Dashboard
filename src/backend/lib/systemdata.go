package lib

import (
	"log"
	"math"

	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/mem"
	"github.com/shirou/gopsutil/process"
)

type MemData struct {
	Percent float64 `json:"percent"`
	Total   uint64  `json:"total"`
	Used    uint64  `json:"used"`
}

type ProcessData struct {
	PID  int32   `json:"pid"`
	Name string  `json:"name"`
	CPU  float64 `json:"cpu"`
	RAM  uint64  `json:"ram"`
}

func CPU() float64 {
	percent, err := cpu.Percent(1000000000, false)
	if err != nil {
		return 0
	}
	return math.Round(percent[0]*100) / 100
}

func RAM() MemData {
	stats, err := mem.VirtualMemory()
	if err != nil {
		return MemData{0, 0, 0}
	}
	return MemData{math.Round(stats.UsedPercent*100) / 100, stats.Total, stats.Used}
}

func Swap() MemData {
	stats, err := mem.SwapMemory()
	if err != nil {
		return MemData{0, 0, 0}
	}
	return MemData{math.Round(stats.UsedPercent*100) / 100, stats.Total, stats.Used}
}

func Processes() []ProcessData {
	var processCPU []ProcessData
	processes, err := process.Processes()
	log.Println("Got processes")
	if err != nil {
		return []ProcessData{}
	}
	for _, element := range processes {
		name, err := element.Name()
		if err != nil {
			continue
		}
		cpu, err := element.CPUPercent()
		if err != nil {
			continue
		}
		ram, err := element.MemoryInfo()
		if err != nil {
			continue
		}
		log.Println("Got process data")
		processCPU = append(processCPU, ProcessData{element.Pid, name, math.Round(cpu*10) / 10, ram.VMS / 1048576})
	}
	log.Println("Done")
	return processCPU
}
