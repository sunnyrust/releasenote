package models

import (
	"errors"
	"fmt"
	"reflect"
	"strings"

	"github.com/astaxie/beego/orm"
)

// Releasenote ...
type Releasenote struct {
	ID      int    `json:"id" orm:"column(id);pk;"`
	Env     string `json:"env" orm:"column(env);null"`
	Owner   string `json:"name,omitempty" orm:"column(owner);null"`
	Name    string `json:"owner,omitempty" orm:"column(name);null"`
	Address string `json:"address" orm:"column(address);null"`
	Version string `json:"version" orm:"column(version);null"`
	Git     string `json:"git" orm:"column(git);null"`
	Docker  string `json:"docker" orm:"column(docker);null"`
	Pubtime string `json:"pubtime,omitempty" orm:"column(pubtime);null"`
	Data    string `json:"data,omitempty" orm:"column(data);null"`
}

// TableName ...
func (t *Releasenote) TableName() string {
	return "releasenote"
}

func init() {
	orm.RegisterModel(new(Releasenote))
}

// AddReleasenote insert a new Releasenote into database and returns
// last inserted Id on success.
func AddReleasenote(m *Releasenote) (id int64, err error) {
	o := orm.NewOrm()
	id, err = o.Insert(m)
	return
}

// GetReleasenoteById retrieves Releasenote by Id. Returns error if
// Id doesn't exist
func GetReleasenoteById(id int) (v *Releasenote, err error) {
	o := orm.NewOrm()
	v = &Releasenote{ID: id}
	if err = o.Read(v); err == nil {
		return v, nil
	}
	return nil, err
}

// GetAllReleasenote retrieves all Releasenote matches certain condition. Returns empty list if
// no records exist
func GetAllReleasenote(query map[string]string, fields []string, sortby []string, order []string,
	offset int64, limit int64) (ml []interface{}, err error) {
	o := orm.NewOrm()
	qs := o.QueryTable(new(Releasenote))
	// query k=v
	for k, v := range query {
		// rewrite dot-notation to Object__Attribute
		k = strings.Replace(k, ".", "__", -1)
		if strings.Contains(k, "isnull") {
			qs = qs.Filter(k, (v == "true" || v == "1"))
		} else {
			qs = qs.Filter(k, v)
		}
	}
	// order by:
	var sortFields []string
	if len(sortby) != 0 {
		if len(sortby) == len(order) {
			// 1) for each sort field, there is an associated order
			for i, v := range sortby {
				orderby := ""
				if order[i] == "desc" {
					orderby = "-" + v
				} else if order[i] == "asc" {
					orderby = v
				} else {
					return nil, errors.New("Error: Invalid order. Must be either [asc|desc]")
				}
				sortFields = append(sortFields, orderby)
			}
			qs = qs.OrderBy(sortFields...)
		} else if len(sortby) != len(order) && len(order) == 1 {
			// 2) there is exactly one order, all the sorted fields will be sorted by this order
			for _, v := range sortby {
				orderby := ""
				if order[0] == "desc" {
					orderby = "-" + v
				} else if order[0] == "asc" {
					orderby = v
				} else {
					return nil, errors.New("Error: Invalid order. Must be either [asc|desc]")
				}
				sortFields = append(sortFields, orderby)
			}
		} else if len(sortby) != len(order) && len(order) != 1 {
			return nil, errors.New("Error: 'sortby', 'order' sizes mismatch or 'order' size is not 1")
		}
	} else {
		if len(order) != 0 {
			return nil, errors.New("Error: unused 'order' fields")
		}
	}

	var l []Releasenote
	qs = qs.OrderBy(sortFields...)
	if _, err = qs.Limit(limit, offset).All(&l, fields...); err == nil {
		if len(fields) == 0 {
			for _, v := range l {
				ml = append(ml, v)
			}
		} else {
			// trim unused fields
			for _, v := range l {
				m := make(map[string]interface{})
				val := reflect.ValueOf(v)
				for _, fname := range fields {
					m[fname] = val.FieldByName(fname).Interface()
				}
				ml = append(ml, m)
			}
		}
		return ml, nil
	}
	return nil, err
}

// UpdateReleasenote updates Releasenote by Id and returns error if
// the record to be updated doesn't exist
func UpdateReleasenoteById(m *Releasenote) (err error) {
	o := orm.NewOrm()
	v := Releasenote{ID: m.ID}
	// ascertain id exists in the database
	if err = o.Read(&v); err == nil {
		var num int64
		if num, err = o.Update(m); err == nil {
			fmt.Println("Number of records updated in database:", num)
		}
	}
	return
}

// DeleteReleasenote deletes Releasenote by Id and returns error if
// the record to be deleted doesn't exist
func DeleteReleasenote(id int) (err error) {
	o := orm.NewOrm()
	v := Releasenote{ID: id}
	// ascertain id exists in the database
	if err = o.Read(&v); err == nil {
		var num int64
		if num, err = o.Delete(&Releasenote{ID: id}); err == nil {
			fmt.Println("Number of records deleted in database:", num)
		}
	}
	return
}
