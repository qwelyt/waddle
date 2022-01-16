import cadquery as cq

space = 19.05
cherryCutOutSize=14.05
cherrySize=14.58
cols = 12
rows = 4

width = cols * space
depth = rows * space
lip=3
height = 10
plateThickness=3

    
guide = cq.Workplane("XY").box(width,depth,height)


bottom = (cq.Workplane("XY")
          .box(width,depth,height)
          .faces("+Z")
          .shell(lip)
          .faces(">Z")
          .wires(cq.selectors.AreaNthSelector(-1))
          .toPending()
          .workplane()
          .extrude(1)
          .faces(">Z")
          .wires()
          .item(0)
          .toPending()
          .workplane()
          .offset2D(-lip/2)
          .cutBlind(-plateThickness)
          )



cherryCut = (cq.Workplane()
             .box(cherryCutOutSize, cherryCutOutSize, plateThickness+2)
             
             # Side cuts
             .faces(">Z")
             .edges("|Y")
             .rect(1,4)
             .cutThruAll()
             
             # Potrusions
             .faces(">Y or <Y")
             .edges("<Z")
             .rect(4,1)
             .extrude(plateThickness+1)
             
             # Add chamfer to potrusions
             .faces(">Y or <Y")
             .edges(">Z")
             .chamfer(0.2,0.4)
             )

# show_object(cherryCut)


plate = (cq.Workplane("XY")
    .box(width,depth,plateThickness)
    .faces(">Z")
    .workplane()
    .rarray(space,space,cols,rows,True)
    .cutEach(lambda loc: cherryCut.val().moved(loc).translate((0,0,-2)))
    )


top = (cq.Workplane("XY")
          .box(width,depth,height/2)
          .faces("-Z")
          .shell(lip)
          .faces(">Z")
          .rect(width-lip,depth-lip)
          .cutThruAll()
           .faces("<Z")
           .wires(cq.selectors.AreaNthSelector(-1))
           .toPending()
           .workplane()
           .extrude(1)
           .faces("<Z")
           .wires()
           .item(0)
           .toPending()
           .workplane()
           .offset2D(-lip/2)
           .cutBlind(-plateThickness)
          )
# Cherry MX reference model by https://github.com/ConstantinoSchillebeeckx/cherry-mx-switch
cherry = cq.importers.importStep('cherry_mx.stp').rotate((0,0,0),(1,0,0),90)
switches = (cq.Workplane()
            .rect(width,depth)
            .rarray(space,space,cols,rows,True)
            .eachpoint(lambda loc: cherry.val().moved(loc))
            .clean()
          )


keyboard = (cq.Assembly()
    .add(bottom, name="bottom", color=cq.Color(0, 0, 1, 1))
    .add(plate, name="plate", color=cq.Color(1,1,0,1))
    .add(switches, name="switches", color=cq.Color(0,1,0,1))
    .add(top, name="top", color=cq.Color(0,1,1,1))
    )
#    .add(guide, color=cq.Color("red"))
    

keyboard\
    .constrain("bottom@faces@>Z", "plate@faces@<Z", "Plane")\
    .constrain("plate@faces@>Z", "top@faces@<Z", "Plane")\
    .constrain("plate@faces@>Z", "switches@faces@<Z", "Plane")


    

keyboard.solve()
#show_object(bottom)
# show_object(plate)
# show_object(top)
show_object(keyboard)



