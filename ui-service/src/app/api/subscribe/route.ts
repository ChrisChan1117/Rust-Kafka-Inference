import { NextResponse } from 'next/server';  
import axios from 'axios';
import type { NextRequest } from 'next/server';  

export async function POST(req: NextRequest) {  
  try {  
    let data = await req.json();
    const res = await axios.post('http://data-service:3010/api/subscribe', data);  

    if (res.status !== 200) {  
      throw new Error('Failed to fetch data');  
    }  
    return NextResponse.json(res.data);  
  } catch (error) {  
    console.error(error);  
    return NextResponse.json(  
      { error: 'Failed to fetch data' },  
      { status: 500 },  
    );  
  }  
}