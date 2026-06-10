const CARICA_ELEMENTARE: f64 = 1.602_176_634e-19; //couloumb
const COSTANTE_DI_COULUMB: f64 = 8.99e9;
const COSTANTE_DIELETTRICA_NEL_VUOTO: f64 = 8.854e-12;

pub fn quantizzazione_carica(n: f64) -> f64 {
    n*CARICA_ELEMENTARE
}

pub fn forza_di_coulumb(q1: f64, q2: f64, d:f64, dielrel: f64) -> f64{
    COSTANTE_DIELETTRICA_NEL_VUOTO*dielrel*((q1*q2)/(d*d))
}