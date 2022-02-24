import cadquery as cq

class waddle: 
    space = 19.05
    cherryCutOutSize = 14.05
    cherrySize = 14.58
    cols = 12
    rows = 4
    
    lip = 25
    fillet = 3
    width = cols * space + lip
    depth = rows * space + lip

    plateW = width - lip/1.5;
    plateD = depth - lip/1.2;
    plateThickness = 3

    
    bottomHeight = 6
    bottomThickness = 3
    bottomInnerW = plateW-10
    bottomInnerD = plateD-8

    topHeight = 15
    
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
    
    def plateNoCuts(self):
        plate = (cq.Workplane("XY")
                .rect(self.plateW, self.plateD)
                .extrude(self.plateThickness)
                .edges("|Z")
                .fillet(self.fillet)
                )
        return plate

    def applyPlateSwitchCuts(self, plate, cut=cherryCut):
        return (plate.workplane()
                .rect(self.plateW, self.plateD, forConstruction=True)
                .rarray(self.space, self.space, self.cols, self.rows, True)
                .cutEach(lambda loc: cut().val().moved(loc).translate((0,0,-0.25)))
                )

    def applyPlateSocketCuts(self, plate, socketX=0, socketY=0):
        return (plate.faces("<Z")
                .edges("<X")
                .translate(((2.54*6)-0.65, socketY, 0))
                .sketch()
                .rarray(2.54, 15.5, 12, 2)
                .circle(1/2)
                .finalize()
                .extrude(self.plateThickness/2, "cut")
                )

    def plate(self, cut=cherryCut, socketX=0, socketY=0):
        plate = self.plateNoCuts()

        # Cut holes for switches
        plate = self.applyPlateSwitchCuts(plate, cut)

        # Place for socket to rest
        plate = self.applyPlateSocketCuts(plate, socketX, socketY)

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
                .workplane(offset=-self.bottomThickness)
                .sketch()
                .rect(self.bottomInnerW, self.bottomInnerD)
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

class waddle_split(waddle):

    def bottom(self):
        bottom = super().bottom()
        botBB = bottom.findSolid().BoundingBox()
        botW2 = botBB.xlen/2

        s1 = cq.Sketch().rect(botW2-super().space, botBB.ymax+0.2)
        s2 = cq.Sketch().rect(botW2+super().space, -botBB.ymin+0.2)

        s1y = s1._faces.BoundingBox().ylen/2
        s2y = s2._faces.BoundingBox().ylen/2

        s1x = s2._faces.BoundingBox().xlen/2
        s2x = s1._faces.BoundingBox().xlen/2

        # Split bottom
        bottom = (bottom
                .faces("<Z")
                .tag("bottom")
                .placeSketch(
                    s1.moved(cq.Location(cq.Vector(s1x,s1y,0)))
                    , s2.moved(cq.Location(cq.Vector(s2x,-s2y,0)))
                    )
                .cutThruAll()
                )


        # Extruded lip
        bottom = (bottom.faces("<Z")
                .vertices(">XY")
                .sketch()
                .push([(0, super().bottomInnerD/2.5)])
                .rect(20,super().bottomInnerD/2)
                .finalize()
                .extrude(-super().bottomThickness/2)
                )
        # Cut to hold extrusion
        bottom = (bottom.faces("<Z")
                .vertices("<Y")
                .vertices(">X")
                .sketch()
                .push([(0, -super().bottomInnerD/2.5)])
                .rect(20,super().bottomInnerD/2)
                .finalize()
                .extrude(-super().bottomThickness/2, "cut")
                )

        # Hole for edge socket
        bottom = (bottom.faces("<Z")
                .vertices("<Y")
                .vertices(">X")
                .translate((0,0,1.5))
                .sketch()
                .push([(-2,-4)])
                .rect(4, 3.5)
                .finalize()
                .extrude(-3.5, "cut")
                )

        # Extrusion for edge socket
        bottom = (bottom.faces("<Z")
                .vertices(">Y")
                .vertices(">X")
                .translate((0,0,1.5))
                .sketch()
                .push([(3/2,4)])
                .rect(3.5,3)
                .finalize()
                .extrude(-3)
                )

        return bottom
      

    def cherryCut(self):
        return super().cherryCut()
    def simpleCut(self):
        return super().simpleCut()
    
    def plate(self, cut=cherryCut, socketX=0, socketY=0):
        plate = super().plateNoCuts()
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
        plate = (plate.faces(">Z")
                .vertices(">XY")
                .sketch()
                .push([(0,-plateBB.ymax/2+0.05)])
                .rect(2,plateBB.ymax-0.1)
                .finalize()
                .extrude(-super().plateThickness/2)
                )
        
        # Cut on top part
        plate = (plate.faces("<Z") # .workplaneFromTagged("top") # does not work
                .vertices(">XY")
                .sketch()
                .push([(0,-plateBB.ymax/2+0.05)])
                .rect(2,plateBB.ymax-0.1)
                .finalize()
                .extrude(super().plateThickness/2, "cut")
                )

        # Extend bottom part
        plate = (plate.faces("<Z")
                .vertices("<Y")
                .vertices(">X")
                .sketch()
                .push([(0,plateBB.ymax/2+0.05)])
                .rect(2,plateBB.ymax+0.1)
                .finalize()
                .extrude(super().plateThickness/2)
                )

        # Cut on bottom part
        plate = (plate.faces(">Z")
                .vertices("<Y")
                .vertices(">X")
                .sketch()
                .push([(0,plateBB.ymax/2+0.05)])
                .rect(2,plateBB.ymax+0.1)
                .finalize()
                .extrude(-super().plateThickness/2, "cut")
                )

        plate = super().applyPlateSwitchCuts(plate, cut)
        plate = super().applyPlateSocketCuts(plate, socketX, socketY)


        return plate

    def top(self):
        top = super().top()
        topBB = top.findSolid().BoundingBox()
        topW2 = topBB.xlen/2

        s1 = cq.Sketch().rect(topW2+super().space/2, topBB.ymax+0.2)
        s2 = cq.Sketch().rect(topW2-super().space/2, -topBB.ymin+0.2)

        s1y = s1._faces.BoundingBox().ylen/2
        s2y = s2._faces.BoundingBox().ylen/2

        s1x = s2._faces.BoundingBox().xlen/2
        s2x = s1._faces.BoundingBox().xlen/2

        # Split top
        top = (top
                .faces("<Z")
                .placeSketch(
                    s1.moved(cq.Location(cq.Vector(s1x,s1y,0)))
                    , s2.moved(cq.Location(cq.Vector(s2x,-s2y,0)))
                    )
                .cutThruAll()
                )
        top = (top
                .faces("<Z")
                .vertices("<Y")
                .vertices(">X")
                .translate((5.5/2,6,3))
                .sketch()
                .rect(5.5,3.5)
                .finalize()
                .extrude(-3.5)
                )

        top = (top
                .faces("<Z")
                .vertices(">Y")
                .vertices(">X")
                .translate((-6/2,-6,3))
                .sketch()
                .rect(6,4)
                .finalize()
                .extrude(-4, "cut")
                )

        return top


class Keyboard:

    def __init__(self
        , bottom=False
        , plate=False
        , switches=False
        , caps=False
        , proMicro=False
        , socket=False
        , top=False
        , magnetsTop=False
        , magnetsBottom=False
        , split = False
        , bothSides = True
            ):
        
        self.bottom = bottom
        self.plate = plate
        self.switches = switches
        self.caps = caps
        self.proMicro = proMicro
        self.socket = socket
        self.top = top
        self.magnetsTop = magnetsTop
        self.magnetsBottom = magnetsBottom
        self.split = split
        self.bothSides = bothSides

    def render(self):
        self._waddle = waddle_split() if self.split else waddle()
        self._bottom = self._waddle.bottom()
        self.bbb = self._bottom.findSolid().BoundingBox()
        self.sock = self._waddle.socket()
        self.sockBox = self.sock.findSolid().BoundingBox()
        self.pmx = self.bbb.xmin+self.sockBox.xmax+6.5
        self.pmy = -self._waddle.space/2
        self.pmz = -3.5
        self.topZ = 0
        self.plateZ = self.topZ + 4.0
        self.switchZ = self.plateZ + self._waddle.plateThickness + 1.4
        self.capsZ = self.switchZ + 9.5

        kb = cq.Assembly(name="waddle")
        if self.bottom:
            self.addBottom(kb)
        if self.plate:
            self.addPlate(kb)
        if self.switches:
            self.addSwitches(kb)
        if self.caps:
            self.addCaps(kb)
        if self.top:
            self.addTop(kb)
        if self.proMicro:
            self.addProMicro(kb)
        if self.socket:
            self.addSocket(kb)
        if self.magnetsTop:
            self.addTopMagnets(kb)
        if self.magnetsBottom:
            self.addBottomMagnets(kb)
        return kb

    def addBottom(self, kb):
        kb.add(self._bottom
                , name  = "bottomL" if self.split else "bottom"
                , color = cq.Color(0.8,0.8,0.8,1)
                , loc   = cq.Location(cq.Vector(0,0,0))
                )
        if self.split and self.bothSides:
            kb.add(self._bottom
                    , name  = "bottomR"
                    , color = cq.Color(0.8,0.4,0.8,1)
                    , loc   = cq.Location(cq.Vector(0,0,0), cq.Vector(0,0,1), 180)
                    )

    def addPlate(self, kb):
        _plate = self._waddle.plate(self._waddle.cherryCut, self.pmx+2, self.pmy)
        kb.add(_plate
                , name  = "plateL" if self.split else "plate"
                , color = cq.Color(1,1,1,1)
                , loc   = cq.Location(cq.Vector(0,0,self.plateZ))
                )
        if self.split and self.bothSides:
            kb.add(_plate
                    , name  = "plateR"
                    , color = cq.Color(0.6, 1, 1, 1)
                    , loc   = cq.Location(cq.Vector(0,0,self.plateZ), cq.Vector(0,0,1), 180)
                    )

    def addTop(self, kb):
        kb.add(self._waddle.top()
                , name  = "topL" if self.split else "top"
                , color = cq.Color(0.1,0.1,0.1,1)
                , loc   = cq.Location(cq.Vector(0,0,self.topZ))
                )

        if self.split and self.bothSides:
            kb.add(self._waddle.top()
                    , name  = "topR"
                    , color = cq.Color(0.3,0.3,0.3,1)
                    , loc   = cq.Location(cq.Vector(0,0,self.topZ), cq.Vector(0,0,1), 180)
                    )

    def addSwitches(self, kb):
        kb.add(self._waddle.switches()
                , name  = "switches"
                , color = cq.Color(0,0,0,1)
                , loc   = cq.Location(cq.Vector(0,0,self.switchZ))
                )

    def addCaps(self, kb):
        kb.add(self._waddle.caps()
                , name  = "caps"
                , color = cq.Color(0.1,0.1,0.1,1)
                , loc   = cq.Location(cq.Vector(0,0,self.capsZ))
                )

    def addProMicro(self, kb):
        kb.add(self._waddle.proMicro("c")
                , name  = "proMicro"
                , color = cq.Color(1,1,0.3,1)
                , loc   = cq.Location(cq.Vector(self.pmx,self.pmy,self.pmz))
                )

    def addSocket(self, kb):
        kb.add(self.sock
                , name  = "socket"
                , color = cq.Color(0.3,0.5,0.3,1)
                , loc   = cq.Location(cq.Vector(self.pmx+1,self.pmy,self.pmz+9.5))
                )

    def addTopMagnets(self, kb):
        kb.add(self._waddle.magnets()
                , name  = "magnetsTop"
                , color = cq.Color(0.5,0.5,0.5,1)
                , loc   = cq.Location(cq.Vector(0,0,0))
            )

    def  addBottomMagnets(self, kb):
        kb.add(self._waddle.magnets()
                , name  = "magnetsBottom"
                , color = cq.Color(0.5,0.5,0.5,1)
                , loc   = cq.Location(cq.Vector(0,0,-5))
             )
     

k = Keyboard(
        bottom=True
        , plate=True
#        , switches=True
#        , caps=True
        , proMicro=True
        , socket=True
        , top=True
        , magnetsTop=True
        , magnetsBottom=True
        , split = True
        , bothSides = True
        ).render()
show_object(k)
k.save("waddle.step")
