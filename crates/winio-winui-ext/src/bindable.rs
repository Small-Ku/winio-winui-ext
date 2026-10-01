use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use windows::core::{IInspectable, Interface, implement};
use winio_winui3::Microsoft::UI::Xaml::Interop::{
    IBindableIterable, IBindableIterable_Impl, IBindableIterator, IBindableIterator_Impl,
};

const E_BOUNDS: windows::core::HRESULT = windows::core::HRESULT(0x8000000Bu32 as i32);

#[implement(IBindableIterator, Agile = false)]
struct BindableIterator {
    items: Arc<[IInspectable]>,
    index: AtomicUsize,
}

impl IBindableIterator_Impl for BindableIterator_Impl {
    fn Current(&self) -> windows::core::Result<IInspectable> {
        self.items
            .get(self.index.load(Ordering::Relaxed))
            .cloned()
            .ok_or_else(|| windows::core::Error::from_hresult(E_BOUNDS))
    }

    fn HasCurrent(&self) -> windows::core::Result<bool> {
        Ok(self.index.load(Ordering::Relaxed) < self.items.len())
    }

    fn MoveNext(&self) -> windows::core::Result<bool> {
        let len = self.items.len();
        let next = self
            .index
            .load(Ordering::Relaxed)
            .saturating_add(1)
            .min(len);
        self.index.store(next, Ordering::Relaxed);
        Ok(next < len)
    }
}

#[implement(IBindableIterable, Agile = false)]
struct BindableIterable {
    items: Arc<[IInspectable]>,
}

impl IBindableIterable_Impl for BindableIterable_Impl {
    fn First(&self) -> windows::core::Result<IBindableIterator> {
        Ok(BindableIterator {
            items: self.items.clone(),
            index: AtomicUsize::new(0),
        }
        .into())
    }
}

pub(crate) fn iterable(items: Vec<IInspectable>) -> windows::core::Result<IInspectable> {
    let iterable: IBindableIterable = BindableIterable {
        items: Arc::from(items),
    }
    .into();
    iterable.cast()
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows::Foundation::{IPropertyValue, PropertyValue};
    use windows::core::HSTRING;

    #[test]
    fn bindable_iterable_yields_items_in_order() {
        let one = PropertyValue::CreateString(&HSTRING::from("one")).unwrap();
        let two = PropertyValue::CreateString(&HSTRING::from("two")).unwrap();
        let source = iterable(vec![one, two]).unwrap();
        let iterable = source.cast::<IBindableIterable>().unwrap();
        let iterator = iterable.First().unwrap();

        assert!(iterator.HasCurrent().unwrap());
        let first = iterator
            .Current()
            .unwrap()
            .cast::<IPropertyValue>()
            .unwrap();
        assert_eq!(first.GetString().unwrap(), HSTRING::from("one"));

        assert!(iterator.MoveNext().unwrap());
        let second = iterator
            .Current()
            .unwrap()
            .cast::<IPropertyValue>()
            .unwrap();
        assert_eq!(second.GetString().unwrap(), HSTRING::from("two"));

        assert!(!iterator.MoveNext().unwrap());
        assert!(!iterator.HasCurrent().unwrap());
    }
}
