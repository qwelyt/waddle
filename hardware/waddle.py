import cadquery as cq

class waddle: 
    space = 19.05
    cherryCutOutSize = 14.05
    cherrySize = 14.58
    cols = 12
    rows = 4
    
    lip = 20
    fillet = 3
    width = cols * space + lip
    depth = rows * space + lip
    
    bottomHeight = 13
    topHeight =9
    
    plateW = width - lip/2;
    plateD = depth - lip/2;
    plateThickness = 3

    def __init__(self):
        pass
    

    def bottom(self):
        return (cq.Workplane("XY")
                .rect(self.width,self.depth)
                .extrude(self.bottomHeight)
                .edges("<Z or Z")
                .fillet(self.fillet)
                
                # Shelf
                .faces(">Z")
                .workplane()
                .rect(self.plateW+1,self.plateD+1)
                .extrude(-self.plateThickness/1.5, "cut")
                .edges("|Z")
                .fillet(self.fillet)
                
                # Hollow
                .faces(">Z")
                .workplane()
                .rect(self.width-self.lip, self.depth-self.lip)
                .extrude(-self.bottomHeight+3, "cut")
                .edges("|Z")
                .fillet(self.fillet)

                # Place for proMicro
                # Cut hole in inner side
                .faces(">Z[1]")
                .edges("<X")
                .workplane(centerOption="CenterOfMass")
                .tag("pmcut")
                .center(35/2-8.7,9.5)
                .rect(35,11)
                .extrude(5, "cut")

                # Mak room för the USB-port
                .workplaneFromTagged("pmcut")
                .center(35/2-8.7,9.5)
                .rect(35,18)
                .extrude(2, "cut")

                # Make room so the socket can reach the pads
                .workplaneFromTagged("pmcut")
                .center(35/2-3,9.5)
                .rect(35,19)
                .extrude(12, "cut")

                # Inset the bottom to keep it in place
                .workplaneFromTagged("pmcut")
                .center(35/2-8.5,9.5)
                .rect(37,19)
                .extrude(-1, "cut")

                # Punch a hole in the outer wall
                .faces("<X")
                .workplane()
                .center(0,2.6)
                .sketch()
                .rect(9.8,4.1)
                .edges("|Z")
                .fillet(100)
                .finalize()
                .extrude(-10, "cut", taper=15)
                #.edges("|X")
                #.fillet(0.49)
                #.edges()
                #.fillet(0.2)
                #.chamfer(0.25)
                )
    
    def top(self):
        return (cq.Workplane("XY")
                .rect(self.width,self.depth)
                .extrude(self.topHeight)
                .edges(">Z or Z")
                .fillet(self.fillet)
                
                # Shelf
                .faces("<Z")
                .workplane()
                .rect(self.plateW+1,self.plateD+1)
                .extrude(-self.plateThickness/1.5, "cut")
                .edges("|Z")
                .fillet(self.fillet)
                
                # Hollow
                .faces("<Z")
                .workplane()
                .rect(self.width-self.lip+1, self.depth-self.lip+1)
                .cutThruAll()
                # .edges("|Z")
                # .fillet(self.fillet)
                )
    
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
    
    def plate(self):
        return (cq.Workplane("XY")
                .rect(self.plateW, self.plateD)
                .extrude(self.plateThickness)
                .edges("|Z")
                .fillet(self.fillet)
                .workplane()
                .rarray(self.space, self.space, self.cols, self.rows, True)
                .cutEach(lambda loc: self.cherryCut().val().moved(loc).translate((0,0,-0.25)))
                )
    
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

def keyboard(bottom=False
        , plate=False
        , switches=False
        , caps=False
        , proMicro=False
        , socket=False
        , top=False
        ):
    
    _waddle = waddle()
    _bottom = _waddle.bottom()
    bbb = _bottom.findSolid().BoundingBox()
    sock = _waddle.socket()
    sockBox = sock.findSolid().BoundingBox()
    pm = bbb.xmin+sockBox.xmax*1.4
    pmz = _waddle.bottomHeight-1.5
    kb = cq.Assembly()

    if bottom:
        kb.add(_bottom
                , name  = "bottom"
                , color = cq.Color(0,0,1,1)
                , loc   = cq.Location(cq.Vector(0,0,0))
                )
    if plate:
        kb.add(_waddle.plate()
                , name  = "plate"
                , color = cq.Color(1,1,0,1)
                , loc   = cq.Location(cq.Vector(0,0,_waddle.bottomHeight-_waddle.plateThickness/2))
                )
    if switches:
        kb.add(_waddle.switches()
                , name  = "switches"
                , color = cq.Color(0,1,0,1)
                , loc   = cq.Location(cq.Vector(0,0,_waddle.bottomHeight+_waddle.plateThickness-0.4))
                )
    if caps:
        kb.add(_waddle.caps()
                , name  = "caps"
                , color = cq.Color(1,0,1,1)
                , loc   = cq.Location(cq.Vector(0,0,_waddle.bottomHeight+_waddle.plateThickness+9.3))
                )
    if proMicro:
        kb.add(_waddle.proMicro("c")
                , name  = "proMicro"
                , color = cq.Color(1,1,0.3,1)
                , loc   = cq.Location(cq.Vector(pm-1.2,_waddle.space/2,pmz-7.5))
                )
    if socket:
        kb.add(sock
                , name  = "socket"
                , color = cq.Color(0.3,0.5,0.3,1)
                , loc   = cq.Location(cq.Vector(pm,_waddle.space/2,pmz))
                )
    if top:
        kb.add(_waddle.top()
                , name  = "top"
                , color = cq.Color(0,1,1,1)
                , loc   = cq.Location(cq.Vector(0,0,_waddle.bottomHeight))
                )
    return kb

k = keyboard(bottom=True
        , plate=False
        , switches=False
        , caps=False
        , proMicro=True
        , socket=False
        , top=False
        )
show_object(k)
# show_object(cq.Workplane(k.toCompound()).rotate((0,0,0),(1,0,0),90).split(keepTop=True))
# show_object(cq.Workplane(k.toCompound()).faces(">X").workplane(-width/2).split(keepTop=True))
#k.save("waddle.step")
