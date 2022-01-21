import cadquery as cq

space = 19.05
cherryCutOutSize=14.05
cherrySize=14.58
cols = 2
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
            .rect(width,depth, forConstruction=True)
            .rarray(space,space,cols,rows,True)
            .eachpoint(lambda loc: cherry.val().moved(loc))
            .combine(glue=True)
          )

# DSA keycap model by Kael Ruland https://www.reddit.com/r/MechanicalKeyboards/comments/5yzsaz/dsa_keycap_i_modeled_print_files_available_in/
dsa1u = cq.importers.importStep('DSA_1u.step').rotate((0,0,0),(1,0,0),90)
caps = (cq.Workplane()
            .rect(width,depth, forConstruction=True)
            .rarray(space,space,cols,rows,True)
            .eachpoint(lambda loc: dsa1u.val().moved(loc))
            .combine(glue=True)
            # .translate((-4,1.6,0))
          )



keyboard = (cq.Assembly()
    .add(bottom, name="bottom", color=cq.Color(0, 0, 1, 1))
    .add(plate, name="plate", color=cq.Color(1,1,0,1))
    .add(switches, name="switches", color=cq.Color(0,1,0,1))
    .add(caps, name="caps", color=cq.Color(1,0,1,1))
    .add(top, name="top", color=cq.Color(0,1,1,1))
    )
#    .add(guide, color=cq.Color("red"))
    

# keyboard\
#     .constrain("bottom@faces@>Z", "plate@faces@<Z", "Plane")\
#     .constrain("plate@faces@>Z", "top@faces@<Z", "Plane")\
#     .constrain("plate@faces@>Z", "switches@faces@<Z", "Plane")\
#     .constrain("plate@edges@>X", "switches@edges@>X", "Plane")

# keyboard\
#     .constrain("bottom@faces@>Z","plate@faces@<Z", "Plane")\
#     .constrain("bottom@faces@>X","plate@faces@>X", "Plane")\
#     .constrain("bottom@faces@>Y","plate@faces@>Y", "Plane")\
#     .constrain("bottom@faces@<X","plate@faces@<X", "Plane")\
#     .constrain("bottom@faces@<Y","plate@faces@<Y", "Plane")\
#     .constrain("top@faces@<Z", "plate@faces@>Z", "Plane")\
#     .constrain("top@faces@<X", "plate@faces@<X", "Plane")\
#     .constrain("top@faces@<Y", "plate@faces@<Y", "Plane")\
#     .constrain("top@faces@>X", "plate@faces@>X", "Plane")\
#     .constrain("top@faces@>Y", "plate@faces@>Y", "Plane")\
#     .constrain("bottom@faces@>Z", "switches@faces@<Z","Plane")\
#     .constrain("bottom@faces@<X", "switches@faces@<X","Plane")\
#     .constrain("bottom@faces@<Y", "switches@faces@<Y","Plane")\
#     .constrain("bottom@faces@>X", "switches@faces@>X","Plane")\
#     .constrain("bottom@faces@>Y", "switches@faces@>Y","Plane")

switches.faces("<X").edges(">Y").vertices(">Z").tag("refpoint")
bottom.faces("<X[-4]").tag("xface")
bottom.faces(">Y[-4]").tag("yface")
# (keyboard
#     .constrain("switches?refpoint", "top@faces@<X","PointInPlane", param=-5)
#     .constrain("switches?refpoint", "top@faces@>Y","PointInPlane", param=-5)
#     .constrain("switches?refpoint", "top@faces@<Z","PointInPlane", param=3)
#     .constrain("top@faces@<X", "switches@faces@>X","Axis")
#     .constrain("top@faces@<Y", "switches@faces@>Y","Axis")
#     .constrain("bottom@faces@<X", "top@faces@>X","Axis")
#     .constrain("bottom@faces@<X", "top@faces@>X","Axis")
#     .constrain("bottom?xface", "plate@faces@<X", "Plane")
#     .constrain("bottom?yface", "plate@faces@>Y", "Plane")
#     .constrain("bottom@faces@>Z", "top@faces@<Z","Plane")
#     .constrain("plate@faces@>Z", "top@faces@<Z","Plane")
# )
(keyboard
 .constrain("bottom@faces@>Z", "plate@faces@<Z", "PointInPlane", param=-1.5)
 .constrain("plate@faces@>Z", "switches@faces@<Z", "PointInPlane", param=-7.7)
  .constrain("switches@faces@>Z", "caps@faces@<Z", "PointInPlane", param=-3)
 .constrain("plate@faces@>Z", "top@faces@<Z", "PointInPlane", param=-1.5)
 
.constrain("plate@faces@<X", "bottom@faces@<X", "Axis")
.constrain("plate@faces@<Y", "bottom@faces@<Y", "Axis")
.constrain("plate@faces@<X", "switches@faces@<X", "Axis")
.constrain("plate@faces@<Y", "switches@faces@<Y", "Axis")
.constrain("plate@faces@<X", "top@faces@<X", "Axis")
.constrain("plate@faces@<Y", "top@faces@<Y", "Axis")
.constrain("plate@faces@<X", "caps@faces@<X", "Axis")
.constrain("plate@faces@<Y", "caps@faces@<Y", "Axis")
 
# .constrain("switches?refpoint", "caps@faces@>X", "PointInPlane", param=-1.1)
# .constrain("switches?refpoint", "caps@faces@<Y", "PointInPlane", param=-1.15)
# .constrain("switches@faces@<X", "caps@faces@<X", "Axis")
# .constrain("switches@faces@<Y", "caps@faces@<Y", "Axis")

# .constrain("switches@faces@<X", "top@faces@<X", "Axis")
# .constrain("switches@faces@<Y", "top@faces@<Y", "Axis")
# .constrain("switches@faces@<X", "bottom@faces@<X", "Axis")
# .constrain("switches@faces@<Y", "bottom@faces@<Y", "Axis")

# .constrain("caps@faces@<X", "top@faces@<X", "Axis")
# .constrain("caps@faces@<Y", "top@faces@<Y", "Axis")
# .constrain("caps@faces@<X", "bottom@faces@<X", "Axis")
# .constrain("caps@faces@<Y", "bottom@faces@<Y", "Axis")
)



keyboard.solve()
#show_object(bottom)
# show_object(plate)
# show_object(top)

# show_object(keyboard)

# cutter = cq.Workplane("XY").box(rows*space+2,cols*space/2,50).translate((0,cols/2,0))))
half = (cq.Workplane(keyboard.toCompound())
        .faces(">Y")
        .workplane()
        .split(keepTop=True)
  )
show_object(half)
# debug(cutter)



