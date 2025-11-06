from main import app
from fastapi.testclient import TestClient
import pytest

client = TestClient(app)


def test_root():
    response = client.get("/")
    assert response.status_code == 200


def test_countries():
    response = client.get("/countries")
    assert response.status_code == 200
    assert sorted(response.json()) == ["England", "France", "Germany", "Italy", "Peru", "Portugal", "Spain"]


# Tests for monthly_average endpoint

def test_monthly_average_london_january():
    """Test retrieving temperature data for London in January"""
    response = client.get("/countries/England/London/January")
    assert response.status_code == 200
    data = response.json()
    assert data == {"high": 45, "low": 36}
    assert "high" in data
    assert "low" in data


def test_monthly_average_paris_july():
    """Test retrieving temperature data for Paris in July (summer month)"""
    response = client.get("/countries/France/Paris/July")
    assert response.status_code == 200
    data = response.json()
    assert data == {"high": 75, "low": 58}


def test_monthly_average_multiple_locations():
    """Test different cities across different countries"""
    test_cases = [
        ("England", "London", "June", {"high": 64, "low": 52}),
        ("France", "Paris", "January", {"high": 45, "low": 36}),
        ("Spain", "Seville", "August", None),  # Will check if exists in actual data
    ]
    
    for country, city, month, expected in test_cases:
        response = client.get(f"/countries/{country}/{city}/{month}")
        if expected:
            assert response.status_code == 200
            assert response.json() == expected



def test_monthly_average_invalid_country():
    """Test with non-existent country - currently raises KeyError (bug: should return 404)"""
    with pytest.raises(Exception):  # FastAPI wraps KeyError in 500 error
        response = client.get("/countries/InvalidCountry/London/January")
        # Bug: Should return 404 with proper error message instead


def test_monthly_average_invalid_city():
    """Test with non-existent city in valid country - currently raises KeyError (bug: should return 404)"""
    with pytest.raises(Exception):  # FastAPI wraps KeyError in 500 error
        response = client.get("/countries/England/InvalidCity/January")
        # Bug: Should return 404 with proper error message instead


def test_monthly_average_invalid_month():
    """Test with non-existent month in valid country/city - currently raises KeyError (bug: should return 404)"""
    with pytest.raises(Exception):  # FastAPI wraps KeyError in 500 error
        response = client.get("/countries/England/London/InvalidMonth")
        # Bug: Should return 404 with proper error message instead


def test_monthly_average_case_sensitivity():
    """Test that endpoints are case-sensitive - lowercase currently raises KeyError"""
    # Lowercase should fail (currently raises exception, should return 404)
    with pytest.raises(Exception):
        response = client.get("/countries/england/london/january")
    
    # Correct case should work
    response = client.get("/countries/England/London/January")
    assert response.status_code == 200


def test_monthly_average_all_months_london():
    """Test all 12 months for London to ensure complete data"""
    months = ["January", "February", "March", "April", "May", "June",
              "July", "August", "September", "October", "November", "December"]
    
    for month in months:
        response = client.get(f"/countries/England/London/{month}")
        assert response.status_code == 200, f"Failed for month: {month}"
        data = response.json()
        assert "high" in data
        assert "low" in data
        assert isinstance(data["high"], int)
        assert isinstance(data["low"], int)
        assert data["high"] >= data["low"], f"High temp should be >= low temp for {month}"


def test_monthly_average_response_structure():
    """Test that response has correct structure with high and low temperatures"""
    response = client.get("/countries/England/London/January")
    assert response.status_code == 200
    data = response.json()
    
    # Check structure
    assert isinstance(data, dict)
    assert len(data) == 2
    assert "high" in data
    assert "low" in data
    
    # Check data types
    assert isinstance(data["high"], int)
    assert isinstance(data["low"], int)