use std::collections::{HashMap, VecDeque};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos_ {
    row: i32,
    col: i32,
    dir: (i32, i32),
    dir_count: u8,
}

impl Pos_ {
    fn new(dir: (i32, i32), row: i32, col: i32, dir_count: u8) -> Self {
        Self {
            row,
            col,
            dir_count,
            dir,
        }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    pos: Pos_,
    score: u32,
}

impl Pos {
    fn new(row: i32, col: i32, dir_count: u8, dir: (i32, i32), score: u32) -> Self {
        Self {
            pos: Pos_::new(dir, row, col, dir_count),
            score,
        }
    }

    fn get_neighbors(self, part_two: bool) -> Vec<Self> {
        let right = Self {
            score: self.score,
            pos: Pos_ {
                row: self.pos.row,
                col: self.pos.col,
                dir_count: 0,
                dir: (self.pos.dir.1, self.pos.dir.0),
            },
        };
        let left = Self {
            score: self.score,
            pos: Pos_ {
                row: self.pos.row,
                col: self.pos.col,
                dir_count: 0,
                dir: (-self.pos.dir.1, -self.pos.dir.0),
            },
        };
        let straight = Self {
            score: self.score,
            pos: Pos_ {
                row: self.pos.row,
                col: self.pos.col,
                dir_count: self.pos.dir_count,
                dir: self.pos.dir,
            },
        };
        if part_two {
            if self.pos.dir_count < 4 {
                vec![straight]
            } else if self.pos.dir_count >= 10 {
                vec![right, left]
            } else {
                vec![right, left, straight]
            }
        } else if self.pos.dir_count >= 3 {
            vec![right, left]
        } else {
            vec![right, left, straight]
        }
    }

    fn move_(&self, part_two: bool) -> Option<Self> {
        if !part_two && self.pos.dir_count >= 3 {
            None
        } else {
            Some(Self {
                score: self.score,
                pos: Pos_ {
                    row: self.pos.row + self.pos.dir.0,
                    col: self.pos.col + self.pos.dir.1,
                    dir_count: self.pos.dir_count + 1,
                    dir: self.pos.dir,
                },
            })
        }
    }
    fn add_score(self, score: u32) -> Self {
        Self {
            score: self.score + score,
            pos: self.pos,
        }
    }
}

pub fn main() {
    let input = "121325426323542412134412222563763214173242535453232325612225164551127635723772145575536566417713247311545541175727733552452212441523653523523
231216223351264655141552632132146473161357374264732366771212156625436662643364737431521763213145311617777525575176251125412656265156636352432
255435624536215242455255274127151752475473257313426654415761317568425352264846453367673257227477673421453656366237631536651114522326245456615
344331433555112552532552441246721161524725237132172162473256673465755557232328644654572412153231541264466241774431162777355546362533344542421
525434363526162414614511663416111745537123243145613227283763543465788327456326856852866874124527274332336777363327232541433222231612255241612
352235344252142631232127357265376676254615135133242727833447765382324487438778656255478645828762521474722353742357252641751351141543313223632
235365554564642142213252263132372377425544636553542833456273556476758848377428287838746647527425711117252167121757242425125161251113264443626
165425632552555165372613266166327523222655423886342562537544327574724656766642523836564685768736434166164652735551562516455571412531311465335
121531436312116267461622426412334716376556665774834833357383344676667323322748475835558366425878324331212135566154435674513154235616216163653
454344321515415445647672543161512554212387736754464372857823786875252667875673864352686688388833573228427444421467132524723143512135562244526
613226242114623136313663545543152317753778652442275543868435823455584488773643782734268233428646834523476537544571432233243333612245253454624
161334323365467755255722513257555111247447826333235664636475334363246553548854742324663775673776676673782743363512746613324113561464351562433
551242656566236736277377461266667667558658874288565746376546236587353778774373782475282584848426676678373323242726212443546316455635653155436
336214616613325457172215621734164865282833245743578454544644268255423686363442453522542852382684442285543288741155321351577752531474223533221
144344123531637514434312127552563257522557774874344644854364727668743855485536325723282665758742642538662557482455224675736631163331511422142
336351626524131641554261754317638358738826684428323377423622538523375362228223672787457562836347625764576734457587474766475747516372652566231
612513241543725162327454351625436332668668856233758464376282784356685584657874346444277467466588865736667748646588425753462255337344555365446
366424126763222316745161123522677374645733584275462735334644344554593365885387847566528547447664424874685455268367223311761364165627461121446
356311142447474655364457372563887674634575568383426367898359693768789377693745759643487776846625334828753572877373875176522444431252331224422
425424324567624576122615622276325327742576743837356844644479575844967796477573537793985633434477625458353733673643674141773331632656722223463
621254765663163614457635662333844655438234278424844747388633859993586587574799568538737743853325275665367223662268284723456517574235121566532
346136661256751245124217883654467857786866448456756588949449553584865546834849685777597945843934338844275462462687574643747327175345551677341
512161274455154234633663764872574643827782625594539793643858755895385735748634556745699898587755353587587723883526358273613757213626117456351
242731413511512266452636286277758466337866238495734956789456574346779889855734834547488794979387533645746648268686345257861622423451374215452
335166447433422657346432388562528287826366355894963778777799438768474485597366944738549768335885458468334823545752547338662336527435117713141
467626137762614433253484867872626742833688586595749979597599774499774673884389699467965985859435965673475766884854322677558417322416245624424
362537221231463666185555537234234367424653394783455736956669948353874555695643679646653874769856983843747377878554684536665271422645536163465
133355763452411133324864436452373446723747645337555838367656555849444569338376987644974885896658553955365284856875454443347341765127212314235
273733633455731656747452578662772427878353837835754448534794744496455567744793635667536654485665395867777686268265783288365376655434725534212
644432565162632358572576325448387286448655489458448646878559887394965346863775833574867349638847993747537897627374726526787353826366471147427
556563226255361765247323432455827446998387988646647354863939867997796476864447747863757865435947479368486357455638582345324573276657463726452
422174721337717272857442783737733588559683868677964486366348486954878587859574487799647793838594987685576489934866876788274372777733371375462
752161217431428388568723358666735554549383436444899376959455964864579887656579945799978683739849675335789896499465323833665742253126552775633
334777552645554748868428457476664434634675764864865543685458978898864876558474664694897655745493969333484579837446532837445274685762554716161
724453764525682552223737538638788799838463689663938997999599644955584644944666486769977558437979677676968864676332477642454568332343114571744
213256326653433462676273747733678457989684878683883759988464894648789767979545777999697899945745688984653454544596464233258876486522145636625
622547345371288747242226232299598884878789874988576976866899799557986984864498485565867466768683657969756443363979523462863762572845234676635
234361754218524365866776553359555657395745646588654869677866459754799689555984978488549799664865378557996867934367767582226253674525436273146
725461275282432446636623863496659574533869994344994554678994979849444579956788647885986946777759437979877497458947546734253247487568744323637
451674534427263687826283534685655553389457477945896845845485748797985869844794875558847745889596443669754948599988389626634674458567321776321
527443232588288574842563884737658733938759658845688945499677996664469957899468888789596675897687894799433478834755945748625523764426571277355
413327625783838684385434664645396393846938778555698554697976684485889764479788894599998486675548676967374995747584798774345774636638842337547
471133737455724564378552487657498857869649566999646599658645479889944766658974484766974987844749775663784496675488669996856248664457636666541
275416538678372775874562883669684354544779585784868856545894745445774767866686495755498456586574457998373563947896936962264246255743634117613
726721558785255224428485856338985895895894978595599844876665957585687689976996678666477984489748777677537496673536495688572254767267565753354
165454645836736554446379748437859635558475849666585546645876767665578986998755788646694984986889757879745845969867369593646727663427388575222
545564574242538573572376863754559656356465466486479647875567798978667779999669867579857885489774759685868473443845595986484243686386845232123
437727747777447645454546944866854985544859849597457489765756759588569976695686687788957988898546467564687698949544396437553666527732437472742
367115673785725744822778539437867558977668665656487666657779676976575876868686998776688564755844988755885877848444837435345735334744868863713
154231384378845435436333346879665634955475645964765798775679956855965897685888755559795895595848747558677976637945799636698472268863723876726
732116326453732742743679844346868563797849585888688889695769657577898667766668595889687788855795968778794787856844789376734268586767366425347
537466534756452332889944985594363567579464986854786669675996956995965899986655798776657877789875695549446985973333996746879848863454378735736
232437823888337527699479375673933555644596944964976878896799977789579799695989869676589996796869744446547678676463794365385975287866545836574
627367827638632257833487968756599579495479858799946669799879959696778576997576959575999998569965575954984785939447896769653826228432882822237
223537546632535267374938566547335549454879975758988678798977879576776859986956766658595698986958958578848466766564589996785372572347788857526
767462554365445862453579685585478846487546855996999778779959876575896779899689975656676589597897476658777687687399568898949973225838357525515
427158354326458335459776869668775468546798566969688958759875797968887699877587768877566566558867988697474796948855943549359532535762286826444
727473624563678642598359846353469567445754654956968558787858589668598687966787868765869896595777745489945767987559549666694333748526728572737
675332588577675777498537776665984784754974678447769996788966969556976987988898659998977968776599574846499549569674367939497675674842682562526
616633734462324766989559479568877855949994744999568655697789999797799697968867759598679966989799946987949497488863549778993553857468428786465
225863848834254684763437333455364984789896494488685778557586996769779996899968987666856968655795984845945877744837377765878864334237263333723
656466528235852463579894579393696645964769474989668989579977858686887779986767768976559996685679595565698476975476954977873333826573723863487
255377484435828279796885878835958665669855456699695565896599897687668998789778966997599877675689664586785676558889696674646653286472743646875
745447273475736483988685464946775766855555866589775586785879687886688876968769788879675679785895895748849486899985939983646375375862755662541
762463265652786795394676999987956644659558986696669597859659787796698667679989796665975869779889769976974586697493437763853976627878473653573
256524446848833848698473648697699998944687948877697765798699666897987786789876896977575795657859568594484658654548968947846598568788363663682
124273355248257646643368469834444579858649647975886779985787988897787796668886788696668976986676778659554758488757543877995469576243468567668
774248454835252458348467636368986954844977968757856577779687799686877786969767996988655766589869556776945784574649978593834693325552643458878
232674784886437298846345348649877498895646569958997577557978788889798796797988688799985987877996885589594688587558473446776677535484267273538
622855828833226287376494589438459699676465566569776865679688686769887887766699878996755665857567595866478994858944846344685899674832387238486
475882247637357668753498444586849898549959695575999995685689987787697898877966668797667566556975896997947448558847969979384533976846745844343
628553637525437239793463598636574895979459876697767996855886967896969789699877679666658586967969896946958999455763855736695738783434346388384
518554888456433663836877639893476956648785697979589986675796699666798876967667776699976758589676988654445467847997658795639478328724567572487
535422734272548767894674876955676979978477978687867698557799989968786976978899987897766985876986768667478558648497767698385476654324656636552
355485777372224736599899667535884798846797698779685695697596866887677767896778869777969776559978796478946456899577573553735744453352385835487
538538734362626739638488766944698944565886859869597755598676996896686868896998879867689857975587957586969554754956698894797754553367526467337
478562664223688544996659356367449569755777768898876577758766669786796797687986799997879899797999576767464565984484889833347359765524734584657
318566285878825468736366765944668498486878476995886676566957989896776999688897987996655556868669786954555999765597989948899875384867383462764
556873627333226234473338883746554566969666956566785576677566776678867996989967789765955586659869794879466859854756599948587739857475427775475
316528428832386727498374889836398866547949844669678999557967797767769767886697869985976687598995568488456957946484798454465934786677665445645
711827865826258229863653693974765548565997545599897898567776699688698778668896876858798586689988877549975444555969443577373969472462426845757
255846247547426838467986347564894658594544785766987569876759988898878688776666967768777955887876755954569689449759468453648573374385234882371
545327574627834757335364964497588997844584478879769775695878589989989768989966866597788876767759964466696697794754497896668476473764828776235
623542268675677528986783476548564985758754975876968787758559856966968769776767698558659957857797887886578848666545849385549633346427676782852
523272367852473835358866377589458545685684767446977955655597859886587999798778858698599666986557877556985899597978638396767487262255282388574
526464537226232253776799357347638799555545556498579666888669677677695667988886698999757896875978958494665989786554456677447786438745867346675
577147534524455465447887948487393449775478485964759679655555889878877785965797688969658775597979469957795684846655587459935354488683868574475
476573744388447423584864793793738687979474964546458656856879977957556596965858776655689965565748458956447677994987584759763968528588724355852
765627353433228375456947843836564677968978758967949997668657898675679896979698566675889695985847675775885449683696794687577937842737586643352
477134587642685728265784483844776466795497794547868798688895955977598795769868757799777956565588895786697794859357379544975528765367763856877
266742482326685677599467374596888546575645989698484699778886886676958799595588877799889858769697674975978644639787996485495458256842325574757
511267643566767533888646333695644346658549984489574685876757869677598886768978755777579757556565598854947664464687733388759522627253848472425
132453735523256254676859739435649744646577849745978576765755576898585578599889897976587769674698699767559987453938553435576682258522368863515
261271326334444562374387646686638477479675989655869968765886857887965969957587656785975744778879468965897886648985574487796442545668557435536
333311652555825272556657854883565378575577657974447779559966885998686989769998795598665749487475768967567457967743633499368744726368236564332
556152854664362567274585587564687735875744594447495957669459795589859798976687768967775954876475944764975646959949897977782647535248678877511
264133385533255388732773947884977359799668959987896575645985977758886987796989957997695676698755444975487666754345698738652872868786345766727
463113176447683582335265393398466858559586884499479697945477646669998577779575676785499475877984488786483549478756845366576537332825275321127
234754217423267847437428693754995579687888987986466865874744755897875878857888866699946656657556949796739636534655639763833526784427583553515
337121135364426334726223568894688765456965954877664774697659467465988684846796557679575457484794849866738548668674443592734238636282848564115
525432728768642268423837647476659966537864884845577948656546694548897564578574576456847644479875989499696347558694977835763227444757773677217
156224463358485728473363843754663638933574949566789679565746758849676455698844667988697995997665785499735885987657974628433833555435685515136
626564231826425747284656538384673854766757349555974958576685795986849894964885544468857766986988954378558377779889746387372527537437461725142
445163665587553553444628539497589763898673745545857459746478755669495459878987475776945787945847867374858357636869769677723374876456457165757
153366452647645476878324567869439896344676947664859995465865987465754895649486979755879996566986669779646658347698956867866267276436816151113
127352244373287438458433247869795949787875964597957497677699867658694688977484588679699786847448937389536364384838565247866454845226161175723
721741643716473372688765645349654653433346574449588877957544676696994485668588556677898759864946868874958453937933688673454722748466623411652
675736624326228287826836766533748773384344658798746846455458798564674548656569788488799848555985667549483833349574762738373276633836476134627
554355724334358242262268673832537853795949493753735499554557564598876545697885759964464675653735897674734494347967552322786248748773433173756
524245531542635544452688778273864537884657748433949889444574468445787474979867568494798686673539667457488746444566645782734865724373616263236
777267215621225823587456728228359948654376758834949464465549786465778584948998557789887889986394343869834563568453768426825343888227144173166
265564113372672575653724842356682744669898357534933347666789565784587947576466449944678587339599744743557488372843638343572776823277543723375
212352523617334254267644573867753683633696466336799678446899434386687976777687834333934593465633459583756789768242338755872352375532647466227
111627741725656332366674333258827826855435645558795857487796886854336863347974873745348869899349886695545664884662678638564436537433222736161
751471263751634114635565276285257754964789786366464984543765398497679333354587347363945366373587745497886436426273645853646845641277744323514
574717513615323754464466666322864236459478738648458834487669937333854886745538895843545845666934588876848228688353378737476634263752771276145
174244655323632474785437732386584262658779585758993364633848866539693757468968548673465698748365568894458263656753622636377414142673221243322
166156571447265174328544282873634644377894647986437555376785653448937566995375636838633878868784898647883365447845536654375453222174632441251
654164557172147564374256368627264444728622377779976798455698345746878373973896765973759947985693879832342646232772324282424464437275452463655
565652532553113233523827884848633483625844846937786747673867368467993338459946345977589487577684334668275362857356337875843212172615654352251
136444124551637421417468456666378884854788583383837673774543339546456453754847436335893836959945773738527375687646736886267674326212777763716
444353742346377224341232334425262657468736873538567459567845689556548746446388464869648779758459726854267866576453566472167257475341222456531
462614525614355533772434583462633832257557638644285865794369797694698863536458549967639547734687288467428853288846886287715632366341631561115
455441432435714326616747438357253222332582425867288445889467835944779599586367765576693398943366488578555332622463247655161333226323675352455
333444526427627456346265253275687327432534244657234864853399846595886679774465998576653787363644627268888657745482657213341134333232217251546
415442524346772367715767346362633778553623736262777663675623548378554545379769474853854558442457463745628676765348376112111266562354717415164
365316216737244653574726463346527547664757422635463777366444786753568754993555528747645573364237467476834244465353114662241533135343526443354
642156535332266156154527631472724633643463332275322654263687536764555433884484872223523578455236337253773846583442363553451141565376121563123
244523631421322355153425713263448623674673364463283834252622474824474765435427683635476382363664338348634362582717422471426736454651564455316
253625236524547371773674334443474228222284888542835387542755752672824826663388384838663866487244658742645257677716551235417613351512356344533
611332423251345151727574654411316523644243288763383785232527743372275338786488442433353623285538557376682745715444327162752542662532463466641
424561223232512512225435257561476517647846267447682737858538454624762665424754623672526763552335854437625663167326414777711372475411244253315
113445234225455575716433756264621114768575746383344332688867545365688287364473386668462857253452527847752367227434333453645536274433363351456
311645443322532425632437732576611256323424536473675555622587885685256723826767245854644433734667687468571222263523247771752351754265251214136
512416342312651622176655526243467562223245588372876637243434578222436572547672245574763847367673234581331713611173336321113414556131451125355
544126134335454312772124477776571372151312722382728682266462285748233358658752327846786837524283276245374515153651776564354246643163351254245
161465163312132356325543532457453137741137674643588543372324648768736672372454445822678234873823421761535355522321756266276365333544564112263
535322143226134621342247743331746616277725262674766583564888573648676748678685644867364857557762545355635147457167542233551416562236121661612
166513621125514212452452465653337671155715633155415735438632523225822753456628472866466783365443766373747533377762526471263564366145253153246
241256124152326322554142667257454522265554757273453374213777577356787834227842242227264715774737665132267716542367135321542134465324433441643
525456252341264533166516163744757274726223464616557543136577647326736286542837871744411672175662567555261523634677477436662463555523615556434".lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap_or(0)).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    let end_row = input.len() - 1;
    let end_col = input[0].len() - 1;
    for part_two in [true, false].iter() {
        // seen is too specific and it makes it pretty slow! we could make it more generic (if
        // we've seen at this count or lower if going in same direction or any count going in a
        // diff direction) to speed things up a bit but that's finnicky to code and I'm lazy!
        let mut seen: HashMap<Pos_, u32> = HashMap::new();
        let mut solution = u32::MAX;
        let mut horizon: VecDeque<Pos> =
            vec![Pos::new(0, 0, 0, (0, 1), 0), Pos::new(0, 0, 0, (1, 0), 0)].into();
        while let Some(pos) = horizon.pop_front() {
            if pos.score >= solution {
                continue;
            }
            let prev_score = seen.get(&pos.pos).unwrap_or(&u32::MAX);
            if prev_score <= &pos.score {
                continue;
            }
            if pos.pos.row == end_row as i32 && pos.pos.col == end_col as i32 {
                if !part_two || pos.pos.dir_count >= 4 {
                    solution = solution.min(pos.score);
                    continue;
                }
            }
            seen.insert(pos.pos, pos.score);
            pos.get_neighbors(*part_two)
                .iter()
                .filter_map(|p| p.move_(*part_two))
                .filter(|p| {
                    p.pos.row >= 0
                        && p.pos.row <= end_row as i32
                        && p.pos.col >= 0
                        && p.pos.col <= end_col as i32
                })
                .map(|p| p.add_score(input[p.pos.row as usize][p.pos.col as usize]))
                .for_each(|p| horizon.push_back(p));
        }
        let part = match part_two {
            true => "two",
            false => "one",
        };
        println!("part {part} {solution}");
    }
}