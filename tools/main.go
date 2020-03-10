package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/360EntSecGroup-Skylar/excelize"
	"github.com/astaxie/beego/orm"
	_ "github.com/mattn/go-sqlite3"
	"sunny.com/rn_excel/models"
)

var (
	sSheet string
)

func doExcelRow(f *excelize.File, bOdd bool, offset int, rn models.Releasenote) {
	sName := `{"border":[
        {"type":"bottom","color":"000000","style":1},
        {"type":"top","color":"000000","style":1},
        {"type":"left","color":"000000","style":1},
        {"type":"right","color":"000000","style":1}
        ],
		"font": {
			"bold": true,
			"size": 12
		},
		"fill": {
			"type": "pattern",
			"color": ["#E0EBF5"],
			"pattern": 1
		},
		"alignment":{"horizontal": "center", "vertical": "center","wrap_text":true}
	}`
	sTitle := `{"border":[
        {"type":"bottom","color":"000000","style":1},
        {"type":"top","color":"000000","style":1},
        {"type":"left","color":"000000","style":1},
        {"type":"right","color":"000000","style":1}
        ],
		"font": {
			"bold": true,
			"size": 12
		},
		"fill": {
			"type": "pattern",
			"color": ["#E0EBF5"],
			"pattern": 1
		},
		"alignment":{"horizontal": "left", "vertical": "center","wrap_text":true}
	}`
	if !bOdd {
		sName = strings.Replace(sName, "#E0EBF5", "#FFC0CB", -1)
		sTitle = strings.Replace(sTitle, "#E0EBF5", "#FFC0CB", -1)
	}
	sAddress := strings.Replace(rn.Address, "<br >", "\r\n", -1)
	sAddress = strings.Replace(sAddress, "<br>", "\r\n", -1)
	sAddress = strings.Replace(sAddress, "<br />", "\r\n", -1)
	styleName, _ := f.NewStyle(sName)
	styleTitle, _ := f.NewStyle(sTitle)
	sOne := strconv.Itoa(1 + offset)
	sTwo := strconv.Itoa(2 + offset)
	sThree := strconv.Itoa(3 + offset)
	sFour := strconv.Itoa(4 + offset)
	sA1 := "A" + sOne
	sA4 := "A" + sFour
	sB1 := "B" + sOne
	sB2 := "B" + sTwo
	sB3 := "B" + sThree
	sB4 := "B" + sFour
	sC1 := "C" + sOne
	sC2 := "C" + sTwo
	sC3 := "C" + sThree
	sC4 := "C" + sFour
	f.SetRowHeight(sSheet, 1+offset, 80)
	f.SetCellValue(sSheet, sA1, rn.Name+"\r\n("+rn.Owner+")")
	f.MergeCell(sSheet, sA1, sA4)
	f.SetCellStyle(sSheet, sA1, sA4, styleName)
	f.SetCellValue(sSheet, sB1, `接口地址`)
	f.SetCellValue(sSheet, sB2, `SW Version`)
	f.SetCellValue(sSheet, sB3, `git Head`)
	f.SetCellValue(sSheet, sB4, `仓库Link`)
	f.SetCellStyle(sSheet, sB1, sB4, styleTitle)
	f.SetCellValue(sSheet, sC1, sAddress)
	f.SetCellValue(sSheet, sC2, rn.Version)
	f.SetCellValue(sSheet, sC3, rn.Git)
	f.SetCellValue(sSheet, sC4, rn.Docker)
	f.SetCellStyle(sSheet, sC1, sC4, styleTitle)

}
func WriteExcelCells(f *excelize.File, rns []models.Releasenote) {

	for index, v := range rns {
		if index%2 == 0 {
			// fmt.Println("====>", index, v)
			doExcelRow(f, true, index*4, v)
		} else {
			doExcelRow(f, false, index*4, v)
		}
	}

}

// CreateExcel ...
func CreateExcel(filename string, rns []interface{}) {
	f := excelize.NewFile()
	index := f.NewSheet(sSheet)
	// Create a new sheet.
	//index := f.NewSheet("Sheet2")
	// Set value of a cell.

	var rn []models.Releasenote
	for _, v := range rns {
		rn = append(rn, v.(models.Releasenote))
	}
	// fmt.Println(rn)
	WriteExcelCells(f, rn)

	f.SetColWidth(sSheet, "A", "A", 35)
	f.SetColWidth(sSheet, "B", "B", 25)
	f.SetColWidth(sSheet, "C", "C", 70)
	// Set active sheet of the workbook.
	f.SetActiveSheet(index)
	if sSheet != "Sheet1" {
		f.DeleteSheet("Sheet1")
	}

	// Save xlsx file by the given path.
	if err := f.SaveAs(filename); err != nil {
		fmt.Println(err)
	}

}

func init() {
	orm.RegisterDriver("sqlite3", orm.DRSqlite)
	orm.RegisterDataBase("default", "sqlite3", "file:releaseNote.db")
	orm.Debug = false
	// orm.RegisterModel(new(models.Releasenote))
}

func getSqlite(env string) (result []interface{}, err error) {
	var fields []string
	var sortby []string
	var order []string
	var query = make(map[string]string)

	if env != "0" {
		query = map[string]string{"env": env}
	}
	l, err := models.GetAllReleasenote(query, fields, sortby, order, 0, -1)
	if err == nil {
		result = l
	} else {
		result = nil
	}
	return
}

func main() {
	// err := orm.RunSyncdb("default", false, false)
	// if err != nil {
	// 	beego.Error(err)
	// }
	// // beego.Run()
	argNum := len(os.Args)
	var result []interface{}

	var err error
	if argNum > 2 {
		result, err = getSqlite(os.Args[2])
		sSheet = "Sheet" + os.Args[2]
	} else {
		sSheet = "Sheet1"
		result, err = getSqlite("0")
	}
	if err == nil {
		if argNum > 1 {
			CreateExcel(os.Args[1], result)
		}
	} else {
		fmt.Println("数据库里面没有数据，没法生成Excel")
	}
}
