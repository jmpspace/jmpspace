// Code generated by protoc-gen-go.
// source: src/ship.proto
// DO NOT EDIT!

/*
Package ship is a generated protocol buffer package.

It is generated from these files:
	src/ship.proto

It has these top-level messages:
	Vessel
	FuelTank
	Engine
	Part
	Beam
	Attach
	Structure
*/
package ship

import proto "github.com/golang/protobuf/proto"
import math "math"

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = math.Inf

type Vessel struct {
	Width            *float64 `protobuf:"fixed64,1,req,name=width" json:"width,omitempty"`
	Length           *float64 `protobuf:"fixed64,2,req,name=length" json:"length,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *Vessel) Reset()         { *m = Vessel{} }
func (m *Vessel) String() string { return proto.CompactTextString(m) }
func (*Vessel) ProtoMessage()    {}

func (m *Vessel) GetWidth() float64 {
	if m != nil && m.Width != nil {
		return *m.Width
	}
	return 0
}

func (m *Vessel) GetLength() float64 {
	if m != nil && m.Length != nil {
		return *m.Length
	}
	return 0
}

type FuelTank struct {
	Radius           *float64 `protobuf:"fixed64,1,req,name=radius" json:"radius,omitempty"`
	Length           *float64 `protobuf:"fixed64,2,req,name=length" json:"length,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *FuelTank) Reset()         { *m = FuelTank{} }
func (m *FuelTank) String() string { return proto.CompactTextString(m) }
func (*FuelTank) ProtoMessage()    {}

func (m *FuelTank) GetRadius() float64 {
	if m != nil && m.Radius != nil {
		return *m.Radius
	}
	return 0
}

func (m *FuelTank) GetLength() float64 {
	if m != nil && m.Length != nil {
		return *m.Length
	}
	return 0
}

type Engine struct {
	Radius           *float64 `protobuf:"fixed64,1,req,name=radius" json:"radius,omitempty"`
	Length           *float64 `protobuf:"fixed64,2,req,name=length" json:"length,omitempty"`
	Group            *int32   `protobuf:"varint,3,req,name=group" json:"group,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *Engine) Reset()         { *m = Engine{} }
func (m *Engine) String() string { return proto.CompactTextString(m) }
func (*Engine) ProtoMessage()    {}

func (m *Engine) GetRadius() float64 {
	if m != nil && m.Radius != nil {
		return *m.Radius
	}
	return 0
}

func (m *Engine) GetLength() float64 {
	if m != nil && m.Length != nil {
		return *m.Length
	}
	return 0
}

func (m *Engine) GetGroup() int32 {
	if m != nil && m.Group != nil {
		return *m.Group
	}
	return 0
}

type Part struct {
	Vessel           *Vessel   `protobuf:"bytes,1,opt,name=vessel" json:"vessel,omitempty"`
	FuelTank         *FuelTank `protobuf:"bytes,2,opt,name=fuelTank" json:"fuelTank,omitempty"`
	Engine           *Engine   `protobuf:"bytes,3,opt,name=engine" json:"engine,omitempty"`
	XXX_unrecognized []byte    `json:"-"`
}

func (m *Part) Reset()         { *m = Part{} }
func (m *Part) String() string { return proto.CompactTextString(m) }
func (*Part) ProtoMessage()    {}

func (m *Part) GetVessel() *Vessel {
	if m != nil {
		return m.Vessel
	}
	return nil
}

func (m *Part) GetFuelTank() *FuelTank {
	if m != nil {
		return m.FuelTank
	}
	return nil
}

func (m *Part) GetEngine() *Engine {
	if m != nil {
		return m.Engine
	}
	return nil
}

type Beam struct {
	Length           *float64 `protobuf:"fixed64,1,req,name=length" json:"length,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *Beam) Reset()         { *m = Beam{} }
func (m *Beam) String() string { return proto.CompactTextString(m) }
func (*Beam) ProtoMessage()    {}

func (m *Beam) GetLength() float64 {
	if m != nil && m.Length != nil {
		return *m.Length
	}
	return 0
}

type Attach struct {
	Id               *int32   `protobuf:"varint,1,req,name=id" json:"id,omitempty"`
	Location         *float64 `protobuf:"fixed64,2,req,name=location" json:"location,omitempty"`
	Rotation         *float64 `protobuf:"fixed64,3,req,name=rotation" json:"rotation,omitempty"`
	Beam             *Beam    `protobuf:"bytes,4,opt,name=beam" json:"beam,omitempty"`
	Part             *Part    `protobuf:"bytes,5,opt,name=part" json:"part,omitempty"`
	XXX_unrecognized []byte   `json:"-"`
}

func (m *Attach) Reset()         { *m = Attach{} }
func (m *Attach) String() string { return proto.CompactTextString(m) }
func (*Attach) ProtoMessage()    {}

func (m *Attach) GetId() int32 {
	if m != nil && m.Id != nil {
		return *m.Id
	}
	return 0
}

func (m *Attach) GetLocation() float64 {
	if m != nil && m.Location != nil {
		return *m.Location
	}
	return 0
}

func (m *Attach) GetRotation() float64 {
	if m != nil && m.Rotation != nil {
		return *m.Rotation
	}
	return 0
}

func (m *Attach) GetBeam() *Beam {
	if m != nil {
		return m.Beam
	}
	return nil
}

func (m *Attach) GetPart() *Part {
	if m != nil {
		return m.Part
	}
	return nil
}

type Structure struct {
	StructureElements []*Attach `protobuf:"bytes,1,rep,name=structureElements" json:"structureElements,omitempty"`
	XXX_unrecognized  []byte    `json:"-"`
}

func (m *Structure) Reset()         { *m = Structure{} }
func (m *Structure) String() string { return proto.CompactTextString(m) }
func (*Structure) ProtoMessage()    {}

func (m *Structure) GetStructureElements() []*Attach {
	if m != nil {
		return m.StructureElements
	}
	return nil
}
