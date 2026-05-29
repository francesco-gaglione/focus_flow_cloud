"use strict";(self.webpackChunkdoc=self.webpackChunkdoc||[]).push([["2661"],{9412(e,t,a){a.d(t,{diagram:()=>k});var i=a(3590),l=a(5871),r=a(3226),s=a(8365),o=a(797),n=a(8731),p=a(7829),c=s.UI.pie,d={sections:new Map,showData:!1,config:c},u=d.sections,g=d.showData,h=structuredClone(c),f=(0,o.K2)(()=>structuredClone(h),"getConfig"),m=(0,o.K2)(()=>{u=new Map,g=d.showData,(0,s.IU)()},"clear"),x=(0,o.K2)(({label:e,value:t})=>{if(t<0)throw Error(`"${e}" has invalid value: ${t}. Negative values are not allowed in pie charts. All slice values must be >= 0.`);u.has(e)||(u.set(e,t),o.Rm.debug(`added new section: ${e}, with value: ${t}`))},"addSection"),w=(0,o.K2)(()=>u,"getSections"),S=(0,o.K2)(e=>{g=e},"setShowData"),$=(0,o.K2)(()=>g,"getShowData"),y={getConfig:f,clear:m,setDiagramTitle:s.ke,getDiagramTitle:s.ab,setAccTitle:s.SV,getAccTitle:s.iN,setAccDescription:s.EI,getAccDescription:s.m7,addSection:x,getSections:w,setShowData:S,getShowData:$},v=(0,o.K2)((e,t)=>{(0,l.S)(e,t),t.setShowData(e.showData),e.sections.map(t.addSection)},"populateDb"),C={parse:(0,o.K2)(async e=>{let t=await (0,n.qg)("pie",e);o.Rm.debug(t),v(t,y)},"parse")},T=(0,o.K2)(e=>`
  .pieCircle{
    stroke: ${e.pieStrokeColor};
    stroke-width : ${e.pieStrokeWidth};
    opacity : ${e.pieOpacity};
  }
  .pieOuterCircle{
    stroke: ${e.pieOuterStrokeColor};
    stroke-width: ${e.pieOuterStrokeWidth};
    fill: none;
  }
  .pieTitleText {
    text-anchor: middle;
    font-size: ${e.pieTitleTextSize};
    fill: ${e.pieTitleTextColor};
    font-family: ${e.fontFamily};
  }
  .slice {
    font-family: ${e.fontFamily};
    fill: ${e.pieSectionTextColor};
    font-size:${e.pieSectionTextSize};
    // fill: white;
  }
  .legend text {
    fill: ${e.pieLegendTextColor};
    font-family: ${e.fontFamily};
    font-size: ${e.pieLegendTextSize};
  }
`,"getStyles"),D=(0,o.K2)(e=>{let t=[...e.values()].reduce((e,t)=>e+t,0),a=[...e.entries()].map(([e,t])=>({label:e,value:t})).filter(e=>e.value/t*100>=1).sort((e,t)=>t.value-e.value);return(0,p.rLf)().value(e=>e.value)(a)},"createPieArcs"),k={parser:C,db:y,renderer:{draw:(0,o.K2)((e,t,a,l)=>{o.Rm.debug("rendering pie chart\n"+e);let n=l.db,c=(0,s.D7)(),d=(0,r.$t)(n.getConfig(),c.pie),u=(0,i.D)(t),g=u.append("g");g.attr("transform","translate(225,225)");let{themeVariables:h}=c,[f]=(0,r.I5)(h.pieOuterStrokeWidth);f??=2;let m=d.textPosition,x=(0,p.JLW)().innerRadius(0).outerRadius(185),w=(0,p.JLW)().innerRadius(185*m).outerRadius(185*m);g.append("circle").attr("cx",0).attr("cy",0).attr("r",185+f/2).attr("class","pieOuterCircle");let S=n.getSections(),$=D(S),y=[h.pie1,h.pie2,h.pie3,h.pie4,h.pie5,h.pie6,h.pie7,h.pie8,h.pie9,h.pie10,h.pie11,h.pie12],v=0;S.forEach(e=>{v+=e});let C=$.filter(e=>"0"!==(e.data.value/v*100).toFixed(0)),T=(0,p.UMr)(y);g.selectAll("mySlices").data(C).enter().append("path").attr("d",x).attr("fill",e=>T(e.data.label)).attr("class","pieCircle"),g.selectAll("mySlices").data(C).enter().append("text").text(e=>(e.data.value/v*100).toFixed(0)+"%").attr("transform",e=>"translate("+w.centroid(e)+")").style("text-anchor","middle").attr("class","slice"),g.append("text").text(n.getDiagramTitle()).attr("x",0).attr("y",-200).attr("class","pieTitleText");let k=[...S.entries()].map(([e,t])=>({label:e,value:t})),b=g.selectAll(".legend").data(k).enter().append("g").attr("class","legend").attr("transform",(e,t)=>"translate(216,"+(22*t-22*k.length/2)+")");b.append("rect").attr("width",18).attr("height",18).style("fill",e=>T(e.label)).style("stroke",e=>T(e.label)),b.append("text").attr("x",22).attr("y",14).text(e=>n.getShowData()?`${e.label} [${e.value}]`:e.label);let K=512+Math.max(...b.selectAll("text").nodes().map(e=>e?.getBoundingClientRect().width??0));u.attr("viewBox",`0 0 ${K} 450`),(0,s.a$)(u,450,K,d.useMaxWidth)},"draw")},styles:T}}}]);