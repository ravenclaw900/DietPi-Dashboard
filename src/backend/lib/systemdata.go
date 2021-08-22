package lib

import (
	"math"

	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/mem"
)

type MemData struct {
	Percent float64 `json:"percent"`
	Total   uint64  `json:"total"`
	Used    uint64  `json:"used"`
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
