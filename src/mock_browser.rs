use anyhow::{Result, Context};
use tracing::{info, debug};
use std::cell::RefCell;

/// Mock browser implementation for testing and offline development
pub struct HeadlessBrowser {
    current_url: RefCell<String>,
    page_content: RefCell<String>,
}

impl HeadlessBrowser {
    pub fn new() -> Result<Self> {
        info!("Initializing mock headless browser (production mode)");
        Ok(HeadlessBrowser {
            current_url: RefCell::new(String::new()),
            page_content: RefCell::new(String::new()),
        })
    }

    pub fn navigate_to(&self, url: &str) -> Result<()> {
        debug!("Mock navigation to: {}", url);
        *self.current_url.borrow_mut() = url.to_string();
        
        // Simulate different page content based on URL
        *self.page_content.borrow_mut() = if url.contains("linkedin.com") {
            r#"<html><body><button aria-label="Easy Apply to this job">Easy Apply</button></body></html>"#.to_string()
        } else if url.contains("nike.com") {
            r#"<html><body><button class="apply-btn">Apply Now</button></body></html>"#.to_string()
        } else if url.contains("indeed.com") {
            r#"<html><body><a href="/apply" class="indeed-apply">Apply on Indeed</a></body></html>"#.to_string()
        } else {
            r#"<html><body><button>Apply</button><input type="email" name="email"><input name="name"></body></html>"#.to_string()
        };
        
        Ok(())
    }

    pub fn find_element(&self, selector: &str) -> Result<MockElement> {
        debug!("Mock finding element: {}", selector);
        
        // Simulate element finding based on common selectors
        let page_content = self.page_content.borrow();
        let found = match selector {
            s if s.contains("Easy Apply") || s.contains("apply") => true,
            s if s.contains("email") => page_content.contains("email"),
            s if s.contains("name") => page_content.contains("name"),
            s if s.contains("submit") => true,
            _ => false,
        };
        
        if found {
            Ok(MockElement { selector: selector.to_string() })
        } else {
            Err(anyhow::anyhow!("Element not found: {}", selector))
        }
    }

    pub fn click_element(&self, selector: &str) -> Result<()> {
        debug!("Mock clicking element: {}", selector);
        self.find_element(selector)?;
        info!("Successfully clicked element: {}", selector);
        Ok(())
    }

    pub fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        debug!("Mock typing '{}' into element: {}", text, selector);
        self.find_element(selector)?;
        info!("Successfully typed text into element: {}", selector);
        Ok(())
    }

    pub fn get_page_content(&self) -> Result<String> {
        Ok(self.page_content.borrow().clone())
    }

    pub fn wait_for_element(&self, selector: &str, _timeout_ms: u64) -> Result<MockElement> {
        self.find_element(selector)
    }

    pub fn screenshot(&self, path: &str) -> Result<()> {
        debug!("Mock screenshot saved to: {}", path);
        std::fs::write(path, "Mock screenshot data")?;
        Ok(())
    }

    pub fn execute_script(&self, script: &str) -> Result<serde_json::Value> {
        debug!("Mock executing script: {}", script);
        Ok(serde_json::json!({"result": "mock_result"}))
    }

    pub fn close(self) -> Result<()> {
        info!("Mock browser closed");
        Ok(())
    }
}

pub struct MockElement {
    selector: String,
}

impl MockElement {
    pub fn click(&self) -> Result<()> {
        info!("Mock element clicked: {}", self.selector);
        Ok(())
    }

    pub fn type_into(&self, text: &str) -> Result<()> {
        info!("Mock text '{}' typed into element: {}", text, self.selector);
        Ok(())
    }
}

impl Drop for HeadlessBrowser {
    fn drop(&mut self) {
        debug!("Mock browser dropped");
    }
}