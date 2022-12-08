package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type File struct {
	name string
	size int64
}
type Dir struct {
	name   string
	parent *Dir
	files  []File
	dirs   []*Dir
}

func newBaseDir() *Dir {
	return &Dir{
		name:   "/",
		parent: nil,
		files:  []File{},
		dirs:   []*Dir{},
	}
}
func (self *Dir) addFile(file File) {
	self.files = append(self.files, file)
}
func (self *Dir) addDir(dir Dir) {
	self.dirs = append(self.dirs, &dir)
}
func (self *Dir) addDir2(name string) {
	self.dirs = append(
		self.dirs,
		&Dir{
			name:   name,
			files:  []File{},
			dirs:   []*Dir{},
			parent: self,
		},
	)
}
func (self *Dir) cd(path string) *Dir {
	if path == "/" {
		if self.parent != nil {
			return self.parent.cd(path)
		}
		return self
	}
	if path == ".." {
		if self.parent != nil {
			return self.parent
		}
	}
	for i := 0; i < len(self.dirs); i++ {
		if self.dirs[i].name == path {
			return self.dirs[i]
		}
	}
	return self
}
func (self Dir) print() {
	fmt.Print("\n", self.name)
	fmt.Print("{\n")
	fmt.Print("Files:", self.files)
	for i := 0; i < len(self.dirs); i++ {
		self.dirs[i].print()
	}
	fmt.Print("\n}\n")
}
func (self Dir) getSize() int64 {
	var size int64 = 0
	for _, file := range self.files {
		size += file.size
	}
	for _, dir := range self.dirs {
		size += dir.getSize()
	}
	return size
}

func (self Dir) sumUnder100K() {
	var sum int64 = 0
	dirsToCheck := self.dirs

	for len(dirsToCheck) > 0 {
		dirs := make([]*Dir, len(dirsToCheck))
		copy(dirs, dirsToCheck)
		dirsToCheck = []*Dir{}
		for _, dir := range dirs {
			dirSize := dir.getSize()
			if dirSize <= 100000 {
				sum += dirSize
			}
			dirsToCheck = append(dirsToCheck, dir.dirs...)
		}
	}
	fmt.Println("Part1: ", sum)
}
func (self Dir) dirToDelete() {
	dirsToCheck := self.dirs
	var totalSizes []int64 = []int64{}

	for len(dirsToCheck) > 0 {
		dirs := make([]*Dir, len(dirsToCheck))
		copy(dirs, dirsToCheck)
		dirsToCheck = []*Dir{}
		for _, dir := range dirs {
			dirSize := dir.getSize()
			totalSizes = append(totalSizes, dirSize)
			dirsToCheck = append(dirsToCheck, dir.dirs...)
		}
	}

	sort.Slice(totalSizes, func(i, j int) bool {
		return totalSizes[i] < totalSizes[j]
	})
	usedSpace := self.getSize()
	var diskSize int64 = 70000000
	var updateSize int64 = 30000000
	for _, size := range totalSizes {
		if (diskSize-updateSize)-(usedSpace-size) >= 0 {
			fmt.Println("Part2: ", size)
			break
		}
	}
}

func main() {
	// readFile, err := os.Open("input_small")
	readFile, err := os.Open("input")
	defer readFile.Close()
	if err != nil {
		panic("Error reading file")
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	mainDir := newBaseDir()
	addingFiles := false

	for fileScanner.Scan() {
		// fmt.Println(mainDir)
		line := strings.Split(fileScanner.Text(), " ")
		if line[0] == "$" {
			addingFiles = false
		}
		if line[0] == "$" && line[1] == "cd" {
			mainDir = mainDir.cd(line[2])
		}
		if line[0] == "$" && line[1] == "ls" {
			addingFiles = true
			continue
		}
		if addingFiles {
			if line[0] == "dir" {
				mainDir.addDir2(line[1])
			} else {
				size, _ := strconv.ParseInt(line[0], 10, 64)
				mainDir.addFile(File{name: line[1], size: size})
			}
		}
	}
	mainDir = mainDir.cd("/")
	// mainDir.print()
	mainDir.sumUnder100K()
	mainDir.dirToDelete()
}
