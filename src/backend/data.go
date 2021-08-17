package data

import (
	"github.com/shirou/gopsutil/cpu"
)

func CPU() int {
	percent, err := cpu.Percent(0, false)
	if err != nil {
		return 0.0
	}
	return int(percent[0])
}
