import cadquery as cq

class waddle_alt: 
    space = 19.05
    cherryCutOutSize = 14.05
    cherrySize = 14.58
    cols = 12
    rows = 4
    
    lip = 25
    fillet = 3
    width = cols * space + lip
    depth = rows * space + lip
    
    bottomHeight = 6
    topHeight = 15
    
    plateW = width - lip/1.5;
    plateD = depth - lip/1.2;
    plateThickness = 3

    def __init__(self):
        pass

    def cherryCut(self):
        return (cq.Workplane("XY")
                 .box(self.cherryCutOutSize, self.cherryCutOutSize, self.plateThickness+2)
    
                 # Side cuts
                 .faces(">Z")
                 .edges("|Y")
                 .rect(1,4)
                 .cutThruAll()
    
                 # Potrusions
                 .faces(">Y or <Y")
                 .edges("<Z")
                 .rect(4,1)
                 .extrude(self.plateThickness+1)
    
                 # Add chamfer to potrusions
                 .faces(">Y or <Y")
                 .edges(">Z")
                 .chamfer(0.2,0.4)
                 )
    def simpleCut(self):
        return (cq.Workplane("XY")
                 .box(self.cherryCutOutSize, self.cherryCutOutSize, self.plateThickness+2)
                 )
    
    def plate(self, cut=cherryCut, socketX=0, socketY=0):
        plate = (cq.Workplane("XY")
                .rect(self.plateW, self.plateD)
                .extrude(self.plateThickness)
                .edges("|Z")
                .fillet(self.fillet)
                )
        # Cut holes for switches
        plate = (plate.workplane()
                .rarray(self.space, self.space, self.cols, self.rows, True)
                .cutEach(lambda loc: cut().val().moved(loc).translate((0,0,-0.25)))
                )

        # Place for socket to rest
        plate = (plate.faces("<Z")
                .edges("<X")
                .translate((6*2.54-0.7, socketY,0))
                .sketch()
                .rarray(2.54,15.5, 12, 2)
                .circle(0.5)
                .finalize()
                .extrude(self.plateThickness/2, "cut")
                )

        return plate
    
    def switches(self):
        # Cherry MX reference model by https://github.com/ConstantinoSchillebeeckx/cherry-mx-switch
        cherry = (cq.importers.importStep('cherry_mx.stp')
              .rotate((0,0,0),(1,0,0),90)
              .translate((0,-1,-1))
              )
    # cherry = cq.Workplane("XY").box(15.6,15.6,3.6+11.6+3.3)
        return (cq.Workplane()
                .rect(self.width+(self.lip/2),self.depth+(self.lip/2), forConstruction=True)
                .rarray(self.space,self.space,self.cols,self.rows,True)
                .eachpoint(lambda loc: cherry.val().moved(loc))
                .combine(glue=True)
              )
    
    def caps(self):
        d=7.97
        # DSA keycap model by Kael Ruland https://www.reddit.com/r/MechanicalKeyboards/comments/5yzsaz/dsa_keycap_i_modeled_print_files_available_in/
        dsa1u = (cq.importers.importStep('DSA_1u.step')
             .rotate((0,0,0),(1,0,0),90)
             .translate((0,0,-d/2))
             )
        # dsa1u = cq.Workplane("XY").box(15.6,15.6,10)
        return (cq.Workplane()
                .rect(self.width,self.depth, forConstruction=True)
                .rarray(self.space,self.space,self.cols,self.rows,True)
                .eachpoint(lambda loc: dsa1u.val().moved(loc))
                .combine(glue=True)
                # .translate((-4,1.6,0))
              )
    
    def proMicro(self, variant="c"):
        return (cq.importers.importStep("proMicro_"+variant+".step")
                #.rotate((0,0,0),(0,1,0),180)
                .rotate((0,0,0),(0,0,1),90)
                .translate((0,0,-1.5))
                )
    
    def socket(self):
        return (cq.importers.importStep("DIL_socket_24pins.step")
                .rotate((0,0,0),(0,1,0),180)
                .rotate((0,0,0),(0,0,1),90)
                )

    def _srect(self, width, depth, chamfer=9):
        return (cq.Sketch()
                .rect(width,depth)
                .vertices()
                .chamfer(chamfer)
                )

    def _chamferedBox(self, width, depth, height, diff=0, chamferA=9, chamferB=4):
        a = self._srect(width, depth, chamferA)
        b = self._srect(width+diff, depth+diff, chamferB)
        return (cq.Workplane("XY")
                .placeSketch(a, b.moved(cq.Location(cq.Vector(0,0,height))))
                .loft()
                )

    def bottom(self):
        bottom = (self._chamferedBox(self.width, self.depth,-self.bottomHeight, -7)
                .edges("<Z")
                .fillet(1)
                )
        minXSide = bottom.faces("<X").val()

        # Inner raised edge to rest the plate on
        bottom = (bottom
                .faces(">Z")
                .workplane()
                .tag("top")
                .sketch()
                .rect(self.plateW+1, self.plateD+1)
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(3)
                )
        
        # Gut the bottom
        bottom = (bottom.faces("<Z")
                .workplane(offset=-2)
                .sketch()
                .rect(self.plateW-10, self.plateD-8)
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(-self.bottomHeight*2, "cut")
                )

        # Place magnets
        bottom = (bottom.faces(">Z")
                .workplane(centerOption="CenterOfMass")
                .placeSketch(self.magnetPlacement().moved(cq.Location(cq.Vector((0,0,-3)))))
                .extrude(-5,"cut")
                )
        # Place for proMicro + socket
        bottom = (bottom.faces("<Z")
                .edges("<X")
                .workplane(centerOption="CenterOfMass", offset=-1)
                .center(0,self.space/2)
                .tag("pmcut")
                .placeSketch(cq.Sketch()
                    .rect(36,19)
                    .moved(cq.Location(cq.Vector(17,0,0)))
                    )
                .extrude(-2, "cut")
                )

        # Make room for the socket
        bottom = (bottom.faces("<Z")
                .edges("<X")
                .workplane(centerOption="CenterOfMass", offset=-1)
                .center(0,self.space/2)
                .tag("pmcut")
                .placeSketch(cq.Sketch()
                    .rect(8,19)
                    .moved(cq.Location(cq.Vector(7,0,0)))
                    )
                .extrude(-10, "cut")
                )

                
        # Punch a hole in the wall (leave 1mm wall to be cut after)
        bottom = (bottom.faces("<X[3]")
                .workplane()
                .center(0,3.1)
                .sketch()
                .rect(9.5,3.5)
                .vertices()
                .fillet(1)
                .finalize()
                .extrude(until=minXSide.translate(cq.Vector(1,0,0)), combine="cut", taper=0)
                )

        # Space for board in wall
        bottom = (bottom.workplaneFromTagged("pmcut")
                .center(5.5,0)
                .rect(5,18)
                .extrude(-3, "cut")
                )

        return bottom

    def top(self):
        top = (self._chamferedBox(self.width, self.depth,self.topHeight, -13, 9, 4)
                .edges(">Z")
                .fillet(2)
                )

        # Cut a hole for switches
        top = (top.faces(">Z")
                .workplane()
                .sketch()
                .rect(self.width-self.lip+1, self.depth-self.lip+1)
                .vertices()
                .chamfer(1.3)
                .finalize()
                .cutThruAll()
                )


        # Shelf for plate
        top= (top.faces("<Z")
                .workplane()
                .tag("bottom")
                .sketch()
                .rect(self.plateW+2, self.plateD+2)
                .vertices()
                .fillet(self.fillet)
                .finalize()
                .extrude(-8, "cut")
                )


        if False:
            # Extra spacing so the bottom fits
            top = (top.workplaneFromTagged("bottom")
                    .sketch()
                    .rect(self.plateW+3, self.plateD+3)
                    .vertices()
                    .fillet(self.fillet)
                    .finalize()
                    .extrude(-3.4, "cut")
                    )

        # Placement for magnets for mounting
        top = (top.faces("<Z")
                .workplane()
                .placeSketch(self.magnetPlacement())
                .extrude(-5,"cut")
                )

        return top

    def magnetPlacement(self):
        return (cq.Sketch()
                .rarray(self.width/2.3,self.depth/1.14,3,2)
                .circle(5/2)
                )

    def magnets(self):
        return (cq.Workplane("XY")
                .placeSketch(self.magnetPlacement())
                .extrude(5)
                )

class waddle_alt_split(waddle_alt):

    def bottom(self):
        b = super().bottom()
        return (b.rotate((0,0,0),(0,1,0),90)
                .workplane(centerOption="CenterOfMass", offset=-2)
                .split(keepBottom=True)
                .rotate((0,0,0),(0,1,0),-90)
                )
    def cherryCut(self):
        return super().cherryCut()
    def simpleCut(self):
        return super().simpleCut()
    
    def plate(self, cut=cherryCut, socketX=0, socketY=0):
        plate = super().plate(cut, socketX, socketY)
        plateBB = plate.findSolid().BoundingBox()

        plateW2 = plateBB.xlen/2

        s1 = cq.Sketch().rect(plateW2-super().space, plateBB.ymax+0.2)
        s2 = cq.Sketch().rect(plateW2+super().space, plateBB.ymax+0.2)

        # Split the plate
        plate = (plate
                .placeSketch(
                    s1.moved(cq.Location(cq.Vector(plateW2/2+super().space/2,plateBB.ymax/2,0)))
                    ,s2.moved(cq.Location(cq.Vector(plateW2/2-super().space/2,plateBB.ymin/2,0)))
                    )
                .cutThruAll()
                )


        # Extend top part
        plate = (plate.faces(">Z").tag("top")
                .vertices(">XY")
                .sketch()
                .push([(0,-plateBB.ymax/2+0.05)])
                .rect(2,plateBB.ymax-0.1)
                .finalize()
                .extrude(-super().plateThickness/2)
                )
        
        # Cut on top part
        plate = (plate.faces(">Z") # .workplaneFromTagged("top") # does not work
                .vertices(">XY")
                .sketch()
                .push([(0,-plateBB.ymax/2+0.05)])
                .rect(2,plateBB.ymax-0.1)
                .finalize()
                .extrude(super().plateThickness/2, "cut")
                )

        plate = (plate.faces(">Z")
                .vertices(">>X[-7] and <Y")
                .sketch()
                .rect(10,20)
                .finalize()
                .extrude(30)
                )


        return plate



def keyboard(bottom=False
        , plate=False
        , switches=False
        , caps=False
        , proMicro=False
        , socket=False
        , top=False
        , magnetsTop=False
        , magnetsBottom=False
        ):
    
    _waddle = waddle_alt_split()
    _bottom = _waddle.bottom()
    bbb = _bottom.findSolid().BoundingBox()
    sock = _waddle.socket()
    sockBox = sock.findSolid().BoundingBox()

    pmx = bbb.xmin+sockBox.xmax+6.5
    pmy = -_waddle.space/2
    pmz = -3.5
    _plate = _waddle.plate(_waddle.simpleCut, pmx+2, pmy)

    topZ = 0#_waddle.bottomHeight
    plateZ = topZ + 4.0
    switchZ = plateZ + _waddle.plateThickness + 1.4
    capsZ = switchZ + 9.5


    kb = cq.Assembly()

    if bottom:
        kb.add(_bottom
                , name  = "bottom"
                , color = cq.Color(0.8,0.8,0.8,0.71)
                , loc   = cq.Location(cq.Vector(0,0,0))
                )
    if plate:
        kb.add(_plate
                , name  = "plate"
                , color = cq.Color(1,1,1,1)
                , loc   = cq.Location(cq.Vector(0,0,plateZ))
                )
    if switches:
        kb.add(_waddle.switches()
                , name  = "switches"
                , color = cq.Color(0,0,0,1)
                , loc   = cq.Location(cq.Vector(0,0,switchZ))
                )
    if caps:
        kb.add(_waddle.caps()
                , name  = "caps"
                , color = cq.Color(0.1,0.1,0.1,1)
                , loc   = cq.Location(cq.Vector(0,0,capsZ))
                )
    if proMicro:
        kb.add(_waddle.proMicro("c")
                , name  = "proMicro"
                , color = cq.Color(1,1,0.3,1)
                , loc   = cq.Location(cq.Vector(pmx,pmy,pmz))
                )
    if socket:
        kb.add(sock
                , name  = "socket"
                , color = cq.Color(0.3,0.5,0.3,1)
                , loc   = cq.Location(cq.Vector(pmx+1,pmy,pmz+9.5))
                )
    if top:
        kb.add(_waddle.top()
                , name  = "top"
                , color = cq.Color(0.1,0.1,0.1,1)
                , loc   = cq.Location(cq.Vector(0,0,topZ))
                )

    if magnetsTop:
        kb.add(_waddle.magnets()
                , name  = "magnetsTop"
                , color = cq.Color(0.5,0.5,0.5,1)
                , loc   = cq.Location(cq.Vector(0,0,0))
            )
    if magnetsBottom:
        kb.add(_waddle.magnets()
                , name  = "magnetsBottom"
                , color = cq.Color(0.5,0.5,0.5,1)
                , loc   = cq.Location(cq.Vector(0,0,-5))
            )
    return kb

k = keyboard(bottom=False
        , plate=True
#        , switches=True
#        , caps=True
#        , proMicro=True
#        , socket=True
#        , top=True
#        , magnetsTop=True
#        , magnetsBottom=True
        )
show_object(k)
k.save("waddle_alt.step")
