module sunnyrust.com/rn_excel

go 1.14

require (
	github.com/360EntSecGroup-Skylar/excelize v1.4.1
	github.com/astaxie/beego v1.12.1
	github.com/mattn/go-sqlite3 v2.0.3+incompatible
	github.com/shiena/ansicolor v0.0.0-20151119151921-a422bbe96644 // indirect
	github.com/tealeg/xlsx v1.0.5
	sunny.com/rn_excel/models v0.0.0-00010101000000-000000000000
	sunny.com/rn_excel/util v0.0.0-00010101000000-000000000000
)

replace (
	sunny.com/rn_excel/models => ./models
	sunny.com/rn_excel/util => ./util
)
