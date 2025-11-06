use actix_web::{test, App};
use weather_api::{create_app, models::Temperature};
use serde_json::Value;

/// Test root endpoint - should redirect to /docs
#[actix_web::test]
async fn test_root() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should return 200 (following redirect) or 302 (redirect response)
    assert!(resp.status().is_success() || resp.status().is_redirection());
}

/// Test countries endpoint - should return list of all countries
#[actix_web::test]
async fn test_countries() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/countries").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let countries: Vec<String> = serde_json::from_slice(&body).unwrap();
    
    let mut sorted_countries = countries.clone();
    sorted_countries.sort();
    
    assert_eq!(
        sorted_countries,
        vec!["England", "France", "Germany", "Italy", "Peru", "Portugal", "Spain"]
    );
}

/// Test retrieving temperature data for London in January
#[actix_web::test]
async fn test_monthly_average_london_january() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/England/London/January")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let data: Temperature = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(data.high, 45);
    assert_eq!(data.low, 36);
}

/// Test retrieving temperature data for Paris in July (summer month)
#[actix_web::test]
async fn test_monthly_average_paris_july() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/France/Paris/July")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let data: Temperature = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(data.high, 75);
    assert_eq!(data.low, 58);
}

/// Test different cities across different countries
#[actix_web::test]
async fn test_monthly_average_multiple_locations() {
    let app = test::init_service(create_app()).await;
    
    let test_cases = vec![
        ("England", "London", "June", 64, 52),
        ("France", "Paris", "January", 45, 36),
    ];
    
    for (country, city, month, expected_high, expected_low) in test_cases {
        let uri = format!("/countries/{}/{}/{}", country, city, month);
        let req = test::TestRequest::get().uri(&uri).to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success(), "Failed for {}/{}/{}", country, city, month);
        
        let body = test::read_body(resp).await;
        let data: Temperature = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(data.high, expected_high, "High temp mismatch for {}/{}/{}", country, city, month);
        assert_eq!(data.low, expected_low, "Low temp mismatch for {}/{}/{}", country, city, month);
    }
}

/// Test with non-existent country - should return 404 (improvement over Python which returns 500)
#[actix_web::test]
async fn test_monthly_average_invalid_country() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/InvalidCountry/London/January")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should return 404, not 500 like Python does
    assert_eq!(resp.status().as_u16(), 404);
}

/// Test with non-existent city in valid country - should return 404
#[actix_web::test]
async fn test_monthly_average_invalid_city() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/England/InvalidCity/January")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should return 404, not 500 like Python does
    assert_eq!(resp.status().as_u16(), 404);
}

/// Test with non-existent month in valid country/city - should return 404
#[actix_web::test]
async fn test_monthly_average_invalid_month() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/England/London/InvalidMonth")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should return 404, not 500 like Python does
    assert_eq!(resp.status().as_u16(), 404);
}

/// Test that endpoints are case-sensitive
#[actix_web::test]
async fn test_monthly_average_case_sensitivity() {
    let app = test::init_service(create_app()).await;
    
    // Lowercase should fail with 404
    let req = test::TestRequest::get()
        .uri("/countries/england/london/january")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
    
    // Correct case should work
    let req = test::TestRequest::get()
        .uri("/countries/England/London/January")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

/// Test all 12 months for London to ensure complete data
#[actix_web::test]
async fn test_monthly_average_all_months_london() {
    let app = test::init_service(create_app()).await;
    
    let months = vec![
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    
    for month in months {
        let uri = format!("/countries/England/London/{}", month);
        let req = test::TestRequest::get().uri(&uri).to_request();
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success(), "Failed for month: {}", month);
        
        let body = test::read_body(resp).await;
        let data: Temperature = serde_json::from_slice(&body).unwrap();
        
        assert!(data.high >= data.low, "High temp should be >= low temp for {}", month);
    }
}

/// Test that response has correct structure with high and low temperatures
#[actix_web::test]
async fn test_monthly_average_response_structure() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/England/London/January")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let data: Value = serde_json::from_slice(&body).unwrap();
    
    // Check structure
    assert!(data.is_object());
    assert!(data.get("high").is_some());
    assert!(data.get("low").is_some());
    
    // Check data types
    assert!(data["high"].is_number());
    assert!(data["low"].is_number());
    
    // Verify as Temperature struct
    let temp: Temperature = serde_json::from_slice(&body).unwrap();
    assert_eq!(temp.high, 45);
    assert_eq!(temp.low, 36);
}

/// Test Seville August data (mentioned in Python tests)
#[actix_web::test]
async fn test_monthly_average_seville_august() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/countries/Spain/Seville/August")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    let data: Temperature = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(data.high, 98);
    assert_eq!(data.low, 69);
}

/// Test multiple cities in Portugal
#[actix_web::test]
async fn test_monthly_average_portugal_cities() {
    let app = test::init_service(create_app()).await;
    
    // Test Lisbon
    let req = test::TestRequest::get()
        .uri("/countries/Portugal/Lisbon/July")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    // Test Porto
    let req = test::TestRequest::get()
        .uri("/countries/Portugal/Porto/July")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
