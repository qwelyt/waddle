import cadquery as cq

sideSize=3.5
barWidth = 12.5
bodyWidth = barWidth+(sideSize/2)
legWidth = bodyWidth+1.3
spacing = 2.54
def legs(num):
    return (cq.Workplane("XY")
            # Small section
            .rarray(legWidth, spacing, 2, num)
            .circle(0.5/2)
            .extrude(3.2)

            # Thicker section
            .faces(">Z")
            .workplane()
            .rarray(legWidth, spacing, 2, num)
            .circle(1.35/2)
            .extrude(1.3)
            
            .faces("<Z[1]")
            .chamfer(0.2124)
            )

def body(num):
    body = (cq.Workplane("XY")
            # Sides
            .rarray(bodyWidth, spacing, 2, num)
            .rect(sideSize, spacing)
            .extrude(2.8)

            .faces(">Z")
            .workplane()
            .tag("top")

            # Holes
            .rarray(legWidth, spacing, 2, num)
            .circle(1/2)
            .cutThruAll(True)
            .faces(">Z")
            .edges("not(<X or >X or <Y or >Y or >Y[0])")
            .chamfer((1.8-1)/2)

            .faces("<Z")
            .workplane()
            .tag("base")

            # Bars on each end
            .workplaneFromTagged("base")
            .rarray(1, spacing*num-2.8, 1, 2)
            .rect(barWidth,2.8)
            .extrude(-1.8)

            # Circular things on the bars
            .faces("<Z")
            .workplane()
            .center(-barWidth/2+1.48+0.5,0)
            .rarray(1, spacing*num-2.8, 1, 2, (False,True))
            .circle(1.48/2)
            .extrude(-1.8-0.8)

            # Notch
            .edges(">Y and <Z")
            .workplane(centerOption="CenterOfMass")
            .center(-2.3/2,0)
            .circle(2.3/2)
            .cutThruAll()

            # Text
            .workplaneFromTagged("top")
            .rect(18, 2.54*num, forConstruction=True)
            .edges("<Y")
            .workplane(centerOption="CenterOfMass")
            .center(0,2.8/2)
            .text(str(num*2), 2,-1.1)
            )
    if num > 6:  # middle bar
            body = (body.workplaneFromTagged("base")
            .rect(barWidth,2.8)
            .extrude(-1.8)
            )
    return body#.edges(">Y or <Y or >X or <X").fillet(0.2)

def socket(nPins):
    _legs = legs(nPins)
    h = _legs.findSolid().BoundingBox().zmax
    return (cq.Assembly(name = "Lathed DIL socket "+str(nPins*2)+"pins by qwelyt")
            .add(_legs
                , name = "pins"
                , color = cq.Color(1,0.9,0.7,1)
                )
            .add(body(nPins)
                , name = "body"
                , color = cq.Color(0.1,0.1,0.1,1)
                , loc = cq.Location(cq.Vector(0,0,h))
                )
            )

numLegs = 12
s = socket(numLegs)
#s = legs(numLegs)
#s = body(numLegs)
show_object(s)
#show_object(proMicro())
#show_object(body(12))
s.save("DIL_socket_"+str(numLegs*2)+"pins.step", "STEP")
