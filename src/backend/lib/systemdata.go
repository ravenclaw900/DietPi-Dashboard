package lib

import (
	"math"

	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/mem"
)

func CPU() float64 {
	percent, err := cpu.Percent(1000000000, false)
	if err != nil {
		return 0
	}
	return math.Round(percent[0]*100) / 100
}

func RAM() int {
	stats, err := mem.VirtualMemory()
	if err != nil {
		return 0
	}
	return int(stats.UsedPercent)
}

func Swap() int {
	stats, err := mem.SwapMemory()
	if err != nil {
		return 0
	}
	return int(stats.UsedPercent)
}
