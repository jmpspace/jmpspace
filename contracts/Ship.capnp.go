package jmpspace

// AUTO GENERATED - DO NOT EDIT

import (
	"bufio"
	"bytes"
	"encoding/json"
	C "github.com/glycerine/go-capnproto"
	"io"
	"math"
)

type Part C.Struct
type PartVessel Part
type PartFuelTank Part
type Part_Which uint16

const (
	PART_VESSEL   Part_Which = 0
	PART_FUELTANK Part_Which = 1
)

func NewPart(s *C.Segment) Part            { return Part(s.NewStruct(24, 0)) }
func NewRootPart(s *C.Segment) Part        { return Part(s.NewRootStruct(24, 0)) }
func AutoNewPart(s *C.Segment) Part        { return Part(s.NewStructAR(24, 0)) }
func ReadRootPart(s *C.Segment) Part       { return Part(s.Root(0).ToStruct()) }
func (s Part) Which() Part_Which           { return Part_Which(C.Struct(s).Get16(16)) }
func (s Part) Vessel() PartVessel          { return PartVessel(s) }
func (s Part) SetVessel()                  { C.Struct(s).Set16(16, 0) }
func (s PartVessel) Width() float64        { return math.Float64frombits(C.Struct(s).Get64(0)) }
func (s PartVessel) SetWidth(v float64)    { C.Struct(s).Set64(0, math.Float64bits(v)) }
func (s PartVessel) Length() float64       { return math.Float64frombits(C.Struct(s).Get64(8)) }
func (s PartVessel) SetLength(v float64)   { C.Struct(s).Set64(8, math.Float64bits(v)) }
func (s Part) FuelTank() PartFuelTank      { return PartFuelTank(s) }
func (s Part) SetFuelTank()                { C.Struct(s).Set16(16, 1) }
func (s PartFuelTank) Radius() float64     { return math.Float64frombits(C.Struct(s).Get64(0)) }
func (s PartFuelTank) SetRadius(v float64) { C.Struct(s).Set64(0, math.Float64bits(v)) }
func (s PartFuelTank) Length() float64     { return math.Float64frombits(C.Struct(s).Get64(8)) }
func (s PartFuelTank) SetLength(v float64) { C.Struct(s).Set64(8, math.Float64bits(v)) }
func (s Part) WriteJSON(w io.Writer) error {
	b := bufio.NewWriter(w)
	var err error
	var buf []byte
	_ = buf
	err = b.WriteByte('{')
	if err != nil {
		return err
	}
	if s.Which() == PART_VESSEL {
		_, err = b.WriteString("\"vessel\":")
		if err != nil {
			return err
		}
		{
			s := s.Vessel()
			err = b.WriteByte('{')
			if err != nil {
				return err
			}
			_, err = b.WriteString("\"width\":")
			if err != nil {
				return err
			}
			{
				s := s.Width()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			err = b.WriteByte(',')
			if err != nil {
				return err
			}
			_, err = b.WriteString("\"length\":")
			if err != nil {
				return err
			}
			{
				s := s.Length()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			err = b.WriteByte('}')
			if err != nil {
				return err
			}
		}
	}
	if s.Which() == PART_FUELTANK {
		_, err = b.WriteString("\"fuelTank\":")
		if err != nil {
			return err
		}
		{
			s := s.FuelTank()
			err = b.WriteByte('{')
			if err != nil {
				return err
			}
			_, err = b.WriteString("\"radius\":")
			if err != nil {
				return err
			}
			{
				s := s.Radius()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			err = b.WriteByte(',')
			if err != nil {
				return err
			}
			_, err = b.WriteString("\"length\":")
			if err != nil {
				return err
			}
			{
				s := s.Length()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			err = b.WriteByte('}')
			if err != nil {
				return err
			}
		}
	}
	err = b.WriteByte('}')
	if err != nil {
		return err
	}
	err = b.Flush()
	return err
}
func (s Part) MarshalJSON() ([]byte, error) {
	b := bytes.Buffer{}
	err := s.WriteJSON(&b)
	return b.Bytes(), err
}
func (s Part) WriteCapLit(w io.Writer) error {
	b := bufio.NewWriter(w)
	var err error
	var buf []byte
	_ = buf
	err = b.WriteByte('(')
	if err != nil {
		return err
	}
	if s.Which() == PART_VESSEL {
		_, err = b.WriteString("vessel = ")
		if err != nil {
			return err
		}
		{
			s := s.Vessel()
			err = b.WriteByte('(')
			if err != nil {
				return err
			}
			_, err = b.WriteString("width = ")
			if err != nil {
				return err
			}
			{
				s := s.Width()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			_, err = b.WriteString(", ")
			if err != nil {
				return err
			}
			_, err = b.WriteString("length = ")
			if err != nil {
				return err
			}
			{
				s := s.Length()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			err = b.WriteByte(')')
			if err != nil {
				return err
			}
		}
	}
	if s.Which() == PART_FUELTANK {
		_, err = b.WriteString("fuelTank = ")
		if err != nil {
			return err
		}
		{
			s := s.FuelTank()
			err = b.WriteByte('(')
			if err != nil {
				return err
			}
			_, err = b.WriteString("radius = ")
			if err != nil {
				return err
			}
			{
				s := s.Radius()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			_, err = b.WriteString(", ")
			if err != nil {
				return err
			}
			_, err = b.WriteString("length = ")
			if err != nil {
				return err
			}
			{
				s := s.Length()
				buf, err = json.Marshal(s)
				if err != nil {
					return err
				}
				_, err = b.Write(buf)
				if err != nil {
					return err
				}
			}
			err = b.WriteByte(')')
			if err != nil {
				return err
			}
		}
	}
	err = b.WriteByte(')')
	if err != nil {
		return err
	}
	err = b.Flush()
	return err
}
func (s Part) MarshalCapLit() ([]byte, error) {
	b := bytes.Buffer{}
	err := s.WriteCapLit(&b)
	return b.Bytes(), err
}

type Part_List C.PointerList

func NewPartList(s *C.Segment, sz int) Part_List { return Part_List(s.NewCompositeList(24, 0, sz)) }
func (s Part_List) Len() int                     { return C.PointerList(s).Len() }
func (s Part_List) At(i int) Part                { return Part(C.PointerList(s).At(i).ToStruct()) }
func (s Part_List) ToArray() []Part {
	n := s.Len()
	a := make([]Part, n)
	for i := 0; i < n; i++ {
		a[i] = s.At(i)
	}
	return a
}
func (s Part_List) Set(i int, item Part) { C.PointerList(s).Set(i, C.Object(item)) }
