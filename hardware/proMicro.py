import cadquery as cq

def board(version):
    if version == "micro":
        return (cq.Workplane("XY")
                .rect(18, 33)
                .extrude(1.5)
                
                .faces(">Z")
                .workplane()
                .center(0,-33/2+0.5+0.75)
                .rarray(18-2, 2.54, 2, 12, (True,False))
                .hole(1)
                )
    elif version == "c":
        return (cq.Workplane("XY")
                .rect(18,34.9)
                .extrude(1.5)
                
                .faces(">Z")
                .workplane()
                .center(0,-34.9/2+0.5+0.75)
                .rarray(18-2, 2.54, 2, 12, (True,False))
                .hole(1)

                .translate((0,1.9/2,0))
                )
    else:
        return (cq.Workplane("XY")
                .rect(18,34)
                .extrude(1.5)
                )
def port(version):
    if version == "micro":
        return (cq.Workplane("XY")
                .rect(7,6)
                .extrude(2)
                .edges("<Z and |Y")
                .chamfer(0.8)
                
                .faces(">Y")
                .workplane()
                .center(0,1)
                .rect(6,1)
                # .edges("<Z and |Y")
    
                .extrude(-5, "cut")
                .edges(">Z")
                .chamfer(0.1)
                )
    elif version == "c":
        return (cq.Workplane("XY")
                .center(0,-3)
                .rect(9, 7.35)
                .extrude(3.25)
                
                .faces(">Y")
                .workplane()
                .center(0,1.6)
                .rect(8.8,3)
                .extrude(-6, "cut")
                .edges("|Y")
                .fillet(1)
                
                .faces(">Y")
                .workplane()
                .rect(7,0.5)
                .extrude(-7)
                )

def components(version):
    if version == "micro":
        return (cq.Workplane("XY")
                .rect(18,33, forConstruction=True)
                
                # .blocks nearest the port
                .workplane().tag("base")
                .center(0,33/4)
                .rect(11,3)
                .extrude(1.5)
                
                # MCU
                .workplaneFromTagged("base")
                .center(0,-33/9)
                .rect(11,11)
                .extrude(1)
                
                # Rear blocks
                .workplaneFromTagged("base")
                .center(0,-33/2.7)
                .rect(11,3)
                .extrude(1)
                )
    elif version == "c":
        return (cq.Workplane("XY")
                .rect(18,34.9, forConstruction=True)
                .workplane().tag("base")
                
                # Things beside port
                .workplaneFromTagged("base")
                .center(0,34.9/2.2)
                .rarray(12,1,2,1)
                .rect(1,1.8)
                .extrude(0.5)
                
                # Blocks behind port
                .workplaneFromTagged("base")
                .center(0,34.9/4)
                .rect(9.25, 3.4)
                .extrude(1.25)
                
                .workplaneFromTagged("base")
                .center(0,34.9/7)
                .rect(11.45, 3.3)
                .extrude(2)
                
                # Chip
                .workplaneFromTagged("base")
                .center(0,-34.9/8)
                .rect(7.5,7.5)
                .extrude(1)
                
                # Blocks in the back
                .workplaneFromTagged("base")
                .center(0,-34.9/2.5)
                .rect(11.45, 4)
                .extrude(1)


                .translate((0,1.9/2,0))
                )


def proMicro(version="micro"):
    pcb = board(version)
    d = pcb.findSolid().BoundingBox().ymax
    print(d)
    return (cq.Assembly(name="Arduino_Pro_Micro-USB-" + str(version))
            .add(pcb
                 , name  = "board"
                 , color = cq.Color(0.1,0.6,0.2,1)
                 , loc   = cq.Location(cq.Vector(0, 0, 0))
                 )
            .add(port(version)
                 , name  = "port"
                 , color = cq.Color(0.3,0.5,1,1)
                 , loc   = cq.Location(cq.Vector(0, d, 1.5))
                 )
            .add(components(version)
                 , name  = "components"
                 , color = cq.Color(0.3,0.5,0.2,1)
                 , loc   = cq.Location(cq.Vector(0, 0, 1.5))
                 )
            )
v = "c"
pm = proMicro(v)
show_object(pm)
#show_object(proMicro("c"))
pm.save("proMicro_"+v+".step")
