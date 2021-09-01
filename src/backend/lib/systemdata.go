package lib

import (
	"math"
	"os/exec"
	"strconv"
	"strings"

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

type DPSoftwareData struct {
	ID           int    `json:"id"`
	Installed    bool   `json:"installed"`
	Name         string `json:"name"`
	Description  string `json:"description"`
	Dependencies string `json:"dependencies"`
	Docs         string `json:"docs"`
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
	processes, err := process.Processes()
	processCPU := make([]ProcessData, len(processes))
	if err != nil {
		return []ProcessData{}
	}
	for i, element := range processes {
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
		processCPU[i] = ProcessData{element.Pid, name, math.Round(cpu*10) / 10, ram.VMS / 1048576}
	}
	return processCPU
}

func DPSoftware() []DPSoftwareData {
	out, err := exec.Command("/boot/dietpi/dietpi-software", "list").Output()
	if err != nil {
		return []DPSoftwareData{}
	}
	outArr := strings.Split(string(out), "\n")
	software := make([]DPSoftwareData, len(outArr))
software:
	for index, element := range outArr[4:] {
		var id int
		var installed bool
		var name, desc, depends, docs string
		for in1, el1 := range strings.Split(element, "|") {
			switch in1 {
			case 0:
				id, _ = strconv.Atoi(strings.TrimSpace(strings.TrimPrefix(strings.TrimPrefix(el1, "\033[32m"), "ID")))
			case 1:
				installtmp, _ := strconv.Atoi(strings.TrimPrefix(strings.TrimSpace(el1), "="))
				installed = installtmp > 0
			case 2:
				namedesc := strings.Split(el1, ":")
				name = strings.TrimSpace(namedesc[0])
				desc = strings.TrimPrefix(strings.TrimSuffix(strings.TrimSpace(namedesc[1]), "\033[0m"), "\033[0m \033[90m")
			case 3:
				if strings.Contains(el1, "DISABLED") {
					software[index] = DPSoftwareData{-1, false, "", "", "", ""}
					continue software
				}
				depends = strings.TrimSpace(el1)
			case 4:
				docs = strings.TrimSuffix(strings.TrimPrefix(strings.TrimSpace(el1), "\033[90m"), "\033[0m")
			}
		}
		software[index] = DPSoftwareData{id, installed, name, desc, depends, docs}
	}
	return software[:len(software)-5]
}
