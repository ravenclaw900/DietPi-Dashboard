package lib

import (
	"github.com/shirou/gopsutil/cpu"
	"github.com/shirou/gopsutil/mem"
)

func CPU() int {
	percent, err := cpu.Percent(0, false)
	if err != nil {
		return 0
	}
	return int(percent[0])
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
