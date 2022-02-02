import cadquery as cq

def legs(num):
    return (cq.Workplane("XY")
            .rarray(14, 2.54, 2, num)
            .circle(0.5/2)
            .extrude(3.2)
            .faces(">Z")
            .workplane()
            .rarray(14, 2.54, 2, num)
            .circle(1.35/2)
            .extrude(1.3)
            )

def body(num):
    return (cq.Workplane("XY")
            .rarray(12.5, 2.54, 2, num)
            .rect(3.48, 2.54)
            .extrude(2.8)

            .faces(">Z")
            .workplane().tag("top")
            .rarray(13.6, 2.54, 2, num)
            .circle(0.5)
            .cutThruAll(True)
            .faces(">Z")
            .edges("not(<X or >X or <Y or >Y or >Y[0])")
            .chamfer(0.45)

            .faces("<Z")
            .workplane()
            .tag("base")


            # middle bar
            .rect(11,2.8)
            .extrude(-1.8)

            # end
            .workplaneFromTagged("base")
            .rarray(1, 2.54*num-2.8, 1, 2)
            .rect(11,2.8)
            .extrude(-1.8)

            .faces("<Z")
            .workplane()
            .center(-3.3,0)
            .rarray(1, 2.54*num-2.8, 1, 2, (False,True))
            .circle(1.48/2)
            .extrude(-1.8-0.8)
            
            .edges(">Y and <X")
            .workplane(centerOption="CenterOfMass")
            .center(6.5,0)
            .circle(2.3/2)
            .cutThruAll()

            .workplaneFromTagged("top")
            #.rect(2,2).extrude(2)
            #.center(6.5, 2)
            .rect(10,30, forConstruction=True)
            .edges("<XY and >Z")
            .workplane(centerOption="CenterOfMass")
            .center(0,1.25)
            .text(str(num*2), 2,-1.1)
            )

def socket(nPins):
    _legs = legs(nPins)
    h = _legs.findSolid().BoundingBox().zmax
    return (cq.Assembly(name = "Lathed DIL socket by qwelyt")
            .add(_legs
                , name = "pins"
                , color = cq.Color(1,0,0.4,1)
                )
            .add(body(nPins)
                , name = "body"
                , color = cq.Color(0.1,0.1,0.1,1)
                , loc = cq.Location(cq.Vector(0,0,h))
                )
            )

s = socket(12)
show_object(s)
#show_object(body(12))
